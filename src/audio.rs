use log::{debug, error, info};
use std::io;
use tokio::io::{AsyncReadExt, stdin};

/// SOT (Start of Transcription) marker sequence: null byte + 'SOT' + null byte
const SOT_MARKER: &[u8] = b"\0SOT\0";

/// Result of SOT marker detection
#[derive(Debug, Clone)]
pub struct SotDetectionResult {
    /// Audio data before the SOT marker (ready for transcription)
    pub audio_data: Vec<u8>,
    /// Remaining buffer data after the SOT marker (to be kept for next processing)
    pub remaining_buffer: Vec<u8>,
    /// Whether a complete SOT marker was found
    pub marker_found: bool,
    /// Position where the SOT marker was found (if found)
    pub marker_position: Option<usize>,
}

/// Audio data chunk received from stdin
#[derive(Debug, Clone)]
pub struct AudioChunk {
    /// Raw audio data bytes
    pub data: Vec<u8>,
    /// Sequence number for tracking order
    pub sequence: u64,
    /// Timestamp when chunk was received
    pub timestamp: std::time::Instant,
}

/// Async stdin listener for audio data
///
/// This function creates an async stream that reads audio data chunks from stdin.
/// It handles binary data efficiently and provides proper error handling and logging.
///
/// # Arguments
/// * `mut buffer` - Mutable reference to audio buffer
///
/// # Returns
/// * `Result<Option<AudioChunk>, io::Error>` - Audio chunk if available, None if end of stream, error if failed
pub async fn read_audio_chunk(buffer: &mut Vec<u8>) -> Result<Option<AudioChunk>, io::Error> {
    debug!("Starting audio chunk read operation");
    let stdin = stdin();
    let mut reader = tokio::io::BufReader::new(stdin);

    // Clear buffer for new data
    buffer.clear();
    buffer.reserve(4096); // Reserve space for efficiency
    debug!("Audio buffer cleared and reserved space");

    // Read data into buffer
    let mut temp_buffer = vec![0u8; 4096];
    debug!("Reading data from stdin into buffer");
    match reader.read(&mut temp_buffer).await {
        Ok(0) => {
            // End of stream
            debug!("End of audio stream detected");
            Ok(None)
        }
        Ok(bytes_read) => {
            debug!("Read {} bytes from stdin", bytes_read);
            // Trim buffer to actual bytes read
            temp_buffer.truncate(bytes_read);

            // Create audio chunk
            let chunk = AudioChunk {
                data: temp_buffer,
                sequence: 0, // This will be managed by the caller
                timestamp: std::time::Instant::now(),
            };

            // Log reception info
            debug!("Created audio chunk with {} bytes", chunk.data.len());
            info!("Received audio chunk: {} bytes", chunk.data.len());

            Ok(Some(chunk))
        }
        Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {
            // Read was interrupted, try again
            Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Read interrupted",
            ))
        }
        Err(e) => {
            // Log error and return it
            error!("Error reading from stdin: {}", e);
            Err(e)
        }
    }
}

/// Audio data processor trait for handling received chunks
pub trait AudioProcessor: Send + Sync {
    /// Process an audio chunk
    ///
    /// # Arguments
    /// * `chunk` - The audio chunk to process
    ///
    /// # Returns
    /// * `Result<(), String>` - Ok if successful, error message if failed
    fn process_chunk(&mut self, chunk: &AudioChunk) -> Result<(), String>;

    /// Check if the processor is ready to process a complete audio segment
    ///
    /// # Returns
    /// * `bool` - True if ready to process, false otherwise
    fn is_ready(&self) -> bool;

    /// Get the accumulated audio data
    ///
    /// # Returns
    /// * `&Vec<u8>` - Reference to accumulated audio data
    fn accumulated_data(&self) -> &Vec<u8>;

    /// Clear accumulated data
    fn clear_data(&mut self);
}

/// Simple audio buffer for accumulating chunks
pub struct AudioBuffer {
    buffer: Vec<u8>,
    total_bytes_received: u64,
}

impl AudioBuffer {
    /// Create a new audio buffer
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            total_bytes_received: 0,
        }
    }

    /// Add a chunk to the buffer
    pub fn add_chunk(&mut self, chunk: &AudioChunk) {
        debug!("Adding chunk with {} bytes to buffer", chunk.data.len());
        self.buffer.extend_from_slice(&chunk.data);
        self.total_bytes_received += chunk.data.len() as u64;
        debug!("Total bytes received: {}", self.total_bytes_received);
        info!(
            "Added {} bytes to buffer (total: {})",
            chunk.data.len(),
            self.total_bytes_received
        );
    }

    /// Get the current buffer contents
    pub fn buffer(&self) -> &Vec<u8> {
        &self.buffer
    }

    /// Get the total bytes received
    pub fn total_bytes_received(&self) -> u64 {
        self.total_bytes_received
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.total_bytes_received = 0;
        info!("Audio buffer cleared");
    }

    /// Check if buffer contains enough data for processing
    pub fn has_sufficient_data(&self, min_bytes: usize) -> bool {
        self.buffer.len() >= min_bytes
    }

    /// Detect SOT marker and extract audio data for transcription
    ///
    /// This method scans the buffer for the SOT marker sequence (\0SOT\0) and
    /// separates the audio data into两部分: data before the marker (ready for transcription)
    /// and remaining data after the marker (to be kept for next processing).
    ///
    /// # Returns
    /// * `SotDetectionResult` containing the extracted audio data, remaining buffer,
    ///   and detection status
    pub fn detect_sot_marker(&self) -> SotDetectionResult {
        debug!(
            "Scanning for SOT marker in buffer ({} bytes)",
            self.buffer.len()
        );
        info!(
            "Scanning for SOT marker in buffer ({} bytes)",
            self.buffer.len()
        );

        if self.buffer.len() < SOT_MARKER.len() {
            debug!(
                "Buffer too small to contain SOT marker (need at least {} bytes)",
                SOT_MARKER.len()
            );
            info!(
                "Buffer too small to contain SOT marker (need at least {} bytes)",
                SOT_MARKER.len()
            );
            return SotDetectionResult {
                audio_data: Vec::new(),
                remaining_buffer: self.buffer.clone(),
                marker_found: false,
                marker_position: None,
            };
        }

        // Search for SOT marker from the end of the buffer backwards
        // This allows us to find the last occurrence of the marker
        debug!(
            "Searching for SOT marker in buffer from position {}",
            self.buffer.len() - SOT_MARKER.len()
        );
        for i in (0..=self.buffer.len() - SOT_MARKER.len()).rev() {
            if &self.buffer[i..i + SOT_MARKER.len()] == SOT_MARKER {
                let marker_position = i;
                let audio_data = self.buffer[..marker_position].to_vec();
                let remaining_buffer = self.buffer[marker_position + SOT_MARKER.len()..].to_vec();

                debug!("SOT marker found at position {}", marker_position);
                info!("SOT marker found at position {}", marker_position);
                debug!("Extracted {} bytes for transcription", audio_data.len());
                info!("Extracted {} bytes for transcription", audio_data.len());
                debug!("Remaining buffer: {} bytes", remaining_buffer.len());
                info!("Remaining buffer: {} bytes", remaining_buffer.len());

                return SotDetectionResult {
                    audio_data,
                    remaining_buffer,
                    marker_found: true,
                    marker_position: Some(marker_position),
                };
            }
        }

        // SOT marker not found
        debug!("SOT marker not found in buffer");
        info!("SOT marker not found in buffer");
        SotDetectionResult {
            audio_data: Vec::new(),
            remaining_buffer: self.buffer.clone(),
            marker_found: false,
            marker_position: None,
        }
    }

    /// Process SOT marker detection and update buffer state
    ///
    /// This method detects the SOT marker and updates the buffer with remaining data
    /// if a marker was found. It returns the audio data ready for transcription.
    ///
    /// # Returns
    /// * `Option<Vec<u8>>` - Some(audio_data) if SOT marker found, None otherwise
    pub fn process_sot_marker(&mut self) -> Option<Vec<u8>> {
        let result = self.detect_sot_marker();

        if result.marker_found {
            // Update buffer with remaining data
            self.buffer = result.remaining_buffer;
            self.total_bytes_received = self.buffer.len() as u64;

            info!("Buffer updated with {} remaining bytes", self.buffer.len());

            Some(result.audio_data)
        } else {
            None
        }
    }

    /// Check if buffer contains a complete SOT marker
    ///
    /// # Returns
    /// * `bool` - True if SOT marker is found, false otherwise
    pub fn has_sot_marker(&self) -> bool {
        self.detect_sot_marker().marker_found
    }

    /// Get the minimum buffer size required to contain a SOT marker
    ///
    /// # Returns
    /// * `usize` - Minimum buffer size needed
    pub fn min_buffer_size_for_sot() -> usize {
        SOT_MARKER.len()
    }
}

impl Default for AudioBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioProcessor for AudioBuffer {
    fn process_chunk(&mut self, chunk: &AudioChunk) -> Result<(), String> {
        self.add_chunk(chunk);
        Ok(())
    }

    fn is_ready(&self) -> bool {
        // Check if buffer contains a SOT marker
        self.has_sot_marker()
    }

    fn accumulated_data(&self) -> &Vec<u8> {
        &self.buffer
    }

    fn clear_data(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_buffer_basic() {
        let mut buffer = AudioBuffer::new();

        assert_eq!(buffer.total_bytes_received(), 0);
        assert!(buffer.buffer().is_empty());

        let chunk = AudioChunk {
            data: vec![1, 2, 3, 4],
            sequence: 0,
            timestamp: std::time::Instant::now(),
        };

        buffer.add_chunk(&chunk);

        assert_eq!(buffer.total_bytes_received(), 4);
        assert_eq!(buffer.buffer(), &vec![1, 2, 3, 4]);
        assert!(buffer.has_sufficient_data(2));
        assert!(!buffer.has_sufficient_data(10));
    }

    #[test]
    fn test_audio_buffer_multiple_chunks() {
        let mut buffer = AudioBuffer::new();

        let chunk1 = AudioChunk {
            data: vec![1, 2, 3],
            sequence: 0,
            timestamp: std::time::Instant::now(),
        };

        let chunk2 = AudioChunk {
            data: vec![4, 5, 6],
            sequence: 1,
            timestamp: std::time::Instant::now(),
        };

        buffer.add_chunk(&chunk1);
        buffer.add_chunk(&chunk2);

        assert_eq!(buffer.total_bytes_received(), 6);
        assert_eq!(buffer.buffer(), &vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_audio_buffer_clear() {
        let mut buffer = AudioBuffer::new();

        let chunk = AudioChunk {
            data: vec![1, 2, 3],
            sequence: 0,
            timestamp: std::time::Instant::now(),
        };

        buffer.add_chunk(&chunk);
        assert_eq!(buffer.total_bytes_received(), 3);

        buffer.clear();
        assert_eq!(buffer.total_bytes_received(), 0);
        assert!(buffer.buffer().is_empty());
    }

    #[test]
    fn test_audio_processor_trait() {
        let mut buffer = AudioBuffer::new();

        let chunk = AudioChunk {
            data: vec![1, 2, 3],
            sequence: 0,
            timestamp: std::time::Instant::now(),
        };

        // Test AudioProcessor trait implementation
        assert!(!buffer.is_ready());

        let result = buffer.process_chunk(&chunk);
        assert!(result.is_ok());

        // Should not be ready without SOT marker
        assert!(!buffer.is_ready());
        assert_eq!(buffer.accumulated_data(), &vec![1, 2, 3]);

        buffer.clear_data();
        assert!(buffer.accumulated_data().is_empty());
    }

    #[test]
    fn test_sot_marker_detection() {
        let mut buffer = AudioBuffer::new();

        // Test with empty buffer
        let result = buffer.detect_sot_marker();
        assert!(!result.marker_found);
        assert_eq!(result.audio_data, <Vec<u8>>::new());
        assert_eq!(result.remaining_buffer, Vec::<u8>::new());

        // Test with buffer too small for SOT marker
        buffer.buffer = vec![1, 2, 3];
        let result = buffer.detect_sot_marker();
        assert!(!result.marker_found);
        assert_eq!(result.audio_data, <Vec<u8>>::new());
        assert_eq!(result.remaining_buffer, vec![1, 2, 3]);

        // Test with buffer containing SOT marker at the end
        buffer.buffer = vec![1, 2, 3, 0, b'S', b'O', b'T', 0];
        let result = buffer.detect_sot_marker();
        assert!(result.marker_found);
        assert_eq!(result.audio_data, vec![1, 2, 3]);
        assert!(result.remaining_buffer.is_empty());
        assert_eq!(result.marker_position, Some(3));

        // Test with buffer containing SOT marker in the middle
        buffer.buffer = vec![1, 2, 3, 0, b'S', b'O', b'T', 0, 4, 5, 6];
        let result = buffer.detect_sot_marker();
        assert!(result.marker_found);
        assert_eq!(result.audio_data, vec![1, 2, 3]);
        assert_eq!(result.remaining_buffer, vec![4, 5, 6]);
        assert_eq!(result.marker_position, Some(3));

        // Test with buffer containing multiple SOT markers (should find the last one)
        buffer.buffer = vec![
            1, 2, 0, b'S', b'O', b'T', 0, 3, 4, 0, b'S', b'O', b'T', 0, 5, 6,
        ];
        let result = buffer.detect_sot_marker();
        assert!(result.marker_found);
        assert_eq!(result.audio_data, vec![1, 2, 0, b'S', b'O', b'T', 0, 3, 4]);
        assert_eq!(result.remaining_buffer, vec![5, 6]);
        assert_eq!(result.marker_position, Some(9));

        // Test with buffer that doesn't contain SOT marker
        buffer.buffer = vec![1, 2, 3, 4, 5];
        let result = buffer.detect_sot_marker();
        assert!(!result.marker_found);
        assert!(result.audio_data.is_empty());
        assert_eq!(result.remaining_buffer, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_process_sot_marker() {
        let mut buffer = AudioBuffer::new();

        // Test with no SOT marker
        buffer.buffer = vec![1, 2, 3, 4, 5];
        let audio_data = buffer.process_sot_marker();
        assert!(audio_data.is_none());
        assert_eq!(buffer.buffer(), &vec![1, 2, 3, 4, 5]);

        // Test with SOT marker
        buffer.buffer = vec![1, 2, 3, 0, b'S', b'O', b'T', 0, 4, 5];
        let audio_data = buffer.process_sot_marker();
        assert!(audio_data.is_some());
        assert_eq!(audio_data.unwrap(), vec![1, 2, 3]);
        assert_eq!(buffer.buffer(), &vec![4, 5]);
    }

    #[test]
    fn test_has_sot_marker() {
        let mut buffer = AudioBuffer::new();

        // Test without SOT marker
        buffer.buffer = vec![1, 2, 3];
        assert!(!buffer.has_sot_marker());

        // Test with SOT marker
        buffer.buffer = vec![1, 2, 3, 0, b'S', b'O', b'T', 0];
        assert!(buffer.has_sot_marker());
    }

    #[test]
    fn test_min_buffer_size_for_sot() {
        assert_eq!(AudioBuffer::min_buffer_size_for_sot(), 5); // \0SOT\0 = 5 bytes
    }

    #[test]
    fn test_sot_marker_spanning_chunk_boundaries() {
        // Test SOT marker detection when it spans across chunk boundaries
        let mut buffer = AudioBuffer::new();

        // Add first chunk (partial SOT marker)
        buffer.buffer.extend_from_slice(&[1, 2, 3, 0, b'S']);

        // SOT marker not complete yet
        assert!(!buffer.has_sot_marker());

        // Add second chunk (completes SOT marker)
        buffer.buffer.extend_from_slice(&[b'O', b'T', 0, 4, 5]);

        // Now SOT marker should be found
        assert!(buffer.has_sot_marker());

        let result = buffer.detect_sot_marker();
        assert!(result.marker_found);
        assert_eq!(result.audio_data, vec![1, 2, 3]);
        assert_eq!(result.remaining_buffer, vec![4, 5]);
    }
}
