use crate::transcription;
use log::{debug, error};
use std::io;
use tokio::io::{AsyncReadExt, AsyncBufReadExt, stdin};

/// Complete audio data received from JSON input
#[derive(Debug, Clone)]
pub struct AudioData {
    /// Raw audio data bytes
    pub data: Vec<u8>,
    /// Timestamp when data was received
    pub timestamp: std::time::Instant,
}

/// JSON reader for audio data
///
/// This function reads complete JSON payloads from stdin and parses them.
/// It handles JSON validation and provides proper error handling and logging.
///
/// # Arguments
///
/// # Returns
/// * `Result<Option<AudioData>, String>` - Audio data if available, None if end of stream, error if failed
pub async fn read_json_audio() -> Result<Option<AudioData>, String> {
    debug!("Starting JSON audio data read operation");
    let stdin = stdin();
    let mut reader = tokio::io::BufReader::new(stdin).lines();

    // Read complete JSON payload from stdin
    debug!("Reading JSON payload from stdin on each new line");

    match reader.next_line().await {
        Ok(None) => {
            // End of stream
            debug!("End of JSON stream detected");
            Ok(None)
        }
        Ok(Some(json_buffer)) => {
            debug!("Read {} bytes from stdin", json_buffer.len());

            // Parse JSON payload
            match serde_json::from_str::<transcription::TranscriptionRequest>(&json_buffer) {
                Ok(request) => {
                    debug!("Successfully parsed JSON request");

                    // Extract audio data from JSON
                    match transcription::extract_audio_data(&request) {
                        Ok(audio_data) => {
                            debug!(
                                "Successfully extracted audio data: {} bytes",
                                audio_data.len()
                            );

                            let audio = AudioData {
                                data: audio_data,
                                timestamp: std::time::Instant::now(),
                            };

                            Ok(Some(audio))
                        }
                        Err(e) => {
                            error!("Failed to extract audio data from JSON: {}", e);
                            Err(format!("Failed to extract audio data: {}", e))
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to parse JSON payload: {}", e);
                    Err(format!("Invalid JSON payload: {}", e))
                }
            }
        }
        Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {
            // Read was interrupted, try again
            Err("Read interrupted".to_string())
        }
        Err(e) => {
            // Log error and return it
            error!("Error reading from stdin: {}", e);
            Err(format!("Error reading from stdin: {}", e))
        }
    }
}

/// Audio data processor trait for handling complete audio data
pub trait AudioProcessor: Send + Sync {
    /// Process complete audio data
    ///
    /// # Arguments
    /// * `audio_data` - The complete audio data to process
    ///
    /// # Returns
    /// * `Result<(), String>` - Ok if successful, error message if failed
    fn process_audio(&mut self, audio_data: &AudioData) -> Result<(), String>;

    /// Check if the processor is ready to process audio data
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

/// Simple audio buffer for handling complete audio data
pub struct AudioBuffer {
    audio_data: Option<AudioData>,
    total_bytes_received: u64,
}

impl AudioBuffer {
    /// Create a new audio buffer
    pub fn new() -> Self {
        Self {
            audio_data: None,
            total_bytes_received: 0,
        }
    }

    /// Set complete audio data
    pub fn set_audio_data(&mut self, audio_data: AudioData) {
        debug!("Setting audio data: {} bytes", audio_data.data.len());
        self.total_bytes_received = audio_data.data.len() as u64;
        self.audio_data = Some(audio_data);
    }

    /// Get the current audio data
    pub fn audio_data(&self) -> Option<&AudioData> {
        self.audio_data.as_ref()
    }

    /// Get the total bytes received
    pub fn total_bytes_received(&self) -> u64 {
        self.total_bytes_received
    }

    /// Clear the audio data
    pub fn clear(&mut self) {
        self.audio_data = None;
        self.total_bytes_received = 0;
        debug!("Audio buffer cleared");
    }

    /// Check if buffer contains audio data for processing
    pub fn has_audio_data(&self) -> bool {
        self.audio_data.is_some()
    }

    /// Take the audio data for processing
    ///
    /// # Returns
    /// * `Option<AudioData>` - Some(audio_data) if available, None otherwise
    pub fn take_audio_data(&mut self) -> Option<AudioData> {
        let audio_data = self.audio_data.take();
        if audio_data.is_some() {
            self.total_bytes_received = 0;
            debug!("Took audio data for processing");
        }
        audio_data
    }
}

impl Default for AudioBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioProcessor for AudioBuffer {
    fn process_audio(&mut self, audio_data: &AudioData) -> Result<(), String> {
        self.set_audio_data(audio_data.clone());
        Ok(())
    }

    fn is_ready(&self) -> bool {
        // Check if buffer contains audio data
        self.has_audio_data()
    }

    fn accumulated_data(&self) -> &Vec<u8> {
        if let Some(ref audio_data) = self.audio_data {
            &audio_data.data
        } else {
            static EMPTY_VEC: Vec<u8> = Vec::new();
            &EMPTY_VEC
        }
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
        assert!(!buffer.has_audio_data());

        let audio_data = AudioData {
            data: vec![1, 2, 3, 4],
            timestamp: std::time::Instant::now(),
        };

        buffer.set_audio_data(audio_data);

        assert_eq!(buffer.total_bytes_received(), 4);
        assert!(buffer.has_audio_data());
        assert_eq!(buffer.accumulated_data(), &vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_audio_buffer_take_audio_data() {
        let mut buffer = AudioBuffer::new();

        let audio_data = AudioData {
            data: vec![1, 2, 3, 4],
            timestamp: std::time::Instant::now(),
        };

        buffer.set_audio_data(audio_data);

        assert!(buffer.has_audio_data());
        assert_eq!(buffer.total_bytes_received(), 4);

        let taken_data = buffer.take_audio_data();
        assert!(taken_data.is_some());
        assert_eq!(taken_data.unwrap().data, vec![1, 2, 3, 4]);
        assert!(!buffer.has_audio_data());
        assert_eq!(buffer.total_bytes_received(), 0);
    }

    #[test]
    fn test_audio_buffer_clear() {
        let mut buffer = AudioBuffer::new();

        let audio_data = AudioData {
            data: vec![1, 2, 3],
            timestamp: std::time::Instant::now(),
        };

        buffer.set_audio_data(audio_data);
        assert_eq!(buffer.total_bytes_received(), 3);
        assert!(buffer.has_audio_data());

        buffer.clear();
        assert_eq!(buffer.total_bytes_received(), 0);
        assert!(!buffer.has_audio_data());
    }

    #[test]
    fn test_audio_processor_trait() {
        let mut buffer = AudioBuffer::new();

        let audio_data = AudioData {
            data: vec![1, 2, 3],
            timestamp: std::time::Instant::now(),
        };

        // Test AudioProcessor trait implementation
        assert!(!buffer.is_ready());

        let result = buffer.process_audio(&audio_data);
        assert!(result.is_ok());

        // Should be ready with audio data
        assert!(buffer.is_ready());
        assert_eq!(buffer.accumulated_data(), &vec![1, 2, 3]);

        buffer.clear_data();
        assert!(!buffer.is_ready());
        assert!(buffer.accumulated_data().is_empty());
    }

    // JSON audio processing tests - these would require mocking stdin which is complex
    // The actual functionality is tested through the transcription module tests
}
