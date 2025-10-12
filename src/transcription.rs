use base64::Engine;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

/// JSON request structure for audio transcription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionRequest {
    /// Audio data - can be base64 encoded string or binary data
    pub audio_data: AudioDataFormat,
    /// Transcription options
    pub options: Option<TranscriptionOptions>,
}

/// Audio data format - supports both base64 and binary representations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AudioDataFormat {
    /// Base64-encoded audio data
    Base64 {
        /// Base64 encoded audio string
        data: String,
        /// Format hint (optional)
        #[serde(rename = "format")]
        _format: Option<String>,
    },
    /// Binary audio data as Vec<u8>
    Binary {
        /// Binary audio data
        data: Vec<u8>,
        /// Format hint (optional)
        #[serde(rename = "format")]
        _format: Option<String>,
    },
}

/// Transcription options that can be configured via JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionOptions {
    /// Language code (e.g., "en", "es", "fr")
    pub language: Option<String>,
    /// Whether to translate the text to English (for multilingual models)
    pub translate_to_english: Option<bool>,
    /// Whether to include timestamps in the output
    pub include_timestamps: Option<bool>,
    /// Maximum number of tokens to generate
    pub max_tokens: Option<usize>,
    /// Temperature for sampling (0.0 to 1.0)
    pub temperature: Option<f32>,
    /// Whether to use beam search decoding
    pub use_beam_search: Option<bool>,
    /// Number of beams for beam search
    pub beam_size: Option<i32>,
    /// Whether to suppress blank tokens
    pub suppress_blank: Option<bool>,
    /// Whether to enable word timestamps
    pub word_timestamps: Option<bool>,
}

impl Default for TranscriptionOptions {
    fn default() -> Self {
        Self {
            language: None,
            translate_to_english: Some(false),
            include_timestamps: Some(true),
            max_tokens: None,
            temperature: Some(0.0),
            use_beam_search: Some(true), // Changed to true to match client
            beam_size: Some(5),          // Changed to Some(5) to match client
            suppress_blank: Some(true),
            word_timestamps: Some(false), // Matches client's word_timestamps parameter
        }
    }
}

/// Extract audio data from a TranscriptionRequest
///
/// # Arguments
/// * `request` - The transcription request containing audio data
///
/// # Returns
/// * `Result<Vec<u8>, String>` - Audio data as Vec<u8> on success, error message on failure
pub fn extract_audio_data(request: &TranscriptionRequest) -> Result<Vec<u8>, String> {
    debug!("Extracting audio data from transcription request");

    match &request.audio_data {
        AudioDataFormat::Base64 { data, .. } => {
            debug!("Processing base64-encoded audio data");
            info!(
                "Processing base64-encoded audio data: {} characters",
                data.len()
            );

            // Decode base64 data
            match base64::engine::general_purpose::STANDARD.decode(data) {
                Ok(decoded_data) => {
                    debug!(
                        "Successfully decoded base64 audio data: {} bytes",
                        decoded_data.len()
                    );
                    info!(
                        "Successfully decoded base64 audio data: {} bytes",
                        decoded_data.len()
                    );
                    Ok(decoded_data)
                }
                Err(e) => {
                    error!("Failed to decode base64 audio data: {}", e);
                    Err(format!("Failed to decode base64 audio data: {}", e))
                }
            }
        }
        AudioDataFormat::Binary { data, .. } => {
            debug!("Processing binary audio data");
            info!("Processing binary audio data: {} bytes", data.len());

            // Validate binary data is not empty
            if data.is_empty() {
                error!("Binary audio data is empty");
                return Err("Binary audio data is empty".to_string());
            }

            // Create a copy of the binary data
            let audio_data = data.clone();
            debug!(
                "Successfully processed binary audio data: {} bytes",
                audio_data.len()
            );
            info!(
                "Successfully processed binary audio data: {} bytes",
                audio_data.len()
            );
            Ok(audio_data)
        }
    }
}

/// Update transcription configuration from JSON options
///
/// # Arguments
/// * `config` - Base transcription configuration to update
/// * `options` - JSON options to apply
///
/// # Returns
/// * `TranscriptionConfig` - Updated configuration
pub fn update_config_from_options(
    config: &TranscriptionConfig,
    options: &TranscriptionOptions,
) -> TranscriptionConfig {
    debug!("Updating transcription configuration from JSON options");
    info!(
        "Updating transcription configuration from JSON options: {:?}",
        options
    );

    let mut updated_config = config.clone();

    // Update each field if provided in options
    if let Some(ref language) = options.language {
        updated_config.language = Some(language.clone());
    }

    if let Some(translate_to_english) = options.translate_to_english {
        updated_config.translate_to_english = translate_to_english;
    }

    if let Some(include_timestamps) = options.include_timestamps {
        updated_config.include_timestamps = include_timestamps;
    }

    if let Some(max_tokens) = options.max_tokens {
        updated_config.max_tokens = Some(max_tokens);
    }

    if let Some(temperature) = options.temperature {
        updated_config.temperature = temperature;
    }

    if let Some(use_beam_search) = options.use_beam_search {
        updated_config.use_beam_search = use_beam_search;
    }

    if let Some(beam_size) = options.beam_size {
        updated_config.beam_size = Some(beam_size);
    }

    if let Some(suppress_blank) = options.suppress_blank {
        updated_config.suppress_blank = suppress_blank;
    }

    if let Some(word_timestamps) = options.word_timestamps {
        updated_config.word_timestamps = word_timestamps;
    }

    debug!("Updated transcription configuration: {:?}", updated_config);
    info!("Updated transcription configuration: {:?}", updated_config);

    updated_config
}

/// Transcription configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionConfig {
    /// Language code (e.g., "en", "es", "fr")
    pub language: Option<String>,
    /// Whether to translate the text to English (for multilingual models)
    pub translate_to_english: bool,
    /// Whether to include timestamps in the output
    pub include_timestamps: bool,
    /// Maximum number of tokens to generate
    pub max_tokens: Option<usize>,
    /// Temperature for sampling (0.0 to 1.0)
    pub temperature: f32,
    /// Whether to use beam search decoding
    pub use_beam_search: bool,
    /// Number of beams for beam search
    pub beam_size: Option<i32>,
    /// Whether to suppress blank tokens
    pub suppress_blank: bool,
    /// Whether to enable word timestamps
    pub word_timestamps: bool,
}

impl Default for TranscriptionConfig {
    fn default() -> Self {
        Self {
            language: None,
            translate_to_english: false,
            include_timestamps: false,
            max_tokens: None,
            temperature: 0.0,
            use_beam_search: false,
            beam_size: None,
            suppress_blank: true,
            word_timestamps: false,
        }
    }
}

/// Transcription result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResult {
    /// The transcribed text
    pub text: String,
    /// Language detected (if available)
    pub language: Option<String>,
    /// Segments with timestamps (if enabled)
    pub segments: Option<Vec<TranscriptionSegment>>,
    /// Whether the transcription was completed successfully
    pub success: bool,
    /// Error message if transcription failed
    pub error: Option<String>,
    /// Time taken for transcription
    pub duration_ms: Option<u64>,
}

/// Transcription segment with timing information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TranscriptionSegment {
    /// Start time in seconds
    pub start: f32,
    /// End time in seconds
    pub end: f32,
    /// Text content of the segment
    pub text: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: Option<f32>,
}

/// Transcription error types
#[derive(Debug)]
pub enum TranscriptionError {
    WhisperContextError(String),
    AudioDataError(String),
    TranscriptionFailed(String),
    ConfigurationError(String),
}

impl std::fmt::Display for TranscriptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranscriptionError::WhisperContextError(e) => write!(f, "Whisper context error: {}", e),
            TranscriptionError::AudioDataError(e) => write!(f, "Audio data error: {}", e),
            TranscriptionError::TranscriptionFailed(e) => write!(f, "Transcription failed: {}", e),
            TranscriptionError::ConfigurationError(e) => write!(f, "Configuration error: {}", e),
        }
    }
}

impl std::error::Error for TranscriptionError {}

/// JSON parsing and validation errors
#[derive(Debug)]
pub enum JsonError {
    /// Invalid JSON format
    InvalidJson(String),
    /// Missing required field
    MissingField(String),
    /// Invalid field value
    InvalidFieldValue(String, String),
    /// Invalid base64 encoding
    InvalidBase64(String),
    /// Audio data validation failed
    AudioDataError(String),
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonError::InvalidJson(e) => write!(f, "Invalid JSON: {}", e),
            JsonError::MissingField(field) => write!(f, "Missing required field: {}", field),
            JsonError::InvalidFieldValue(field, value) => {
                write!(f, "Invalid value for field '{}': {}", field, value)
            }
            JsonError::InvalidBase64(e) => write!(f, "Invalid base64 encoding: {}", e),
            JsonError::AudioDataError(e) => write!(f, "Audio data error: {}", e),
        }
    }
}

impl std::error::Error for JsonError {}

/// Validation errors for transcription options
#[derive(Debug)]
pub struct ValidationError {
    /// Field name that failed validation
    pub field: String,
    /// Validation error message
    pub message: String,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(field: &str, message: &str) -> Self {
        Self {
            field: field.to_string(),
            message: message.to_string(),
        }
    }
}

/// Validate transcription options
pub fn validate_transcription_options(
    options: &TranscriptionOptions,
) -> Result<Vec<ValidationError>, JsonError> {
    let mut errors = Vec::new();

    // Validate language code
    if let Some(ref lang) = options.language {
        let valid_languages = [
            "en", "auto", "zh", "de", "es", "ru", "ko", "fr", "ja", "pt", "tr", "pl", "ca",
        ];
        if !valid_languages.contains(&lang.as_str()) {
            errors.push(ValidationError::new(
                "language",
                &format!(
                    "Invalid language code: {}. Valid codes are: {:?}",
                    lang, valid_languages
                ),
            ));
        }
    }

    // Validate temperature range
    if let Some(temperature) = options.temperature {
        if temperature < 0.0 || temperature > 1.0 {
            errors.push(ValidationError::new(
                "temperature",
                "Temperature must be between 0.0 and 1.0",
            ));
        }
    }

    // Validate beam size if specified
    if let Some(beam_size) = options.beam_size {
        if beam_size < 1 {
            errors.push(ValidationError::new(
                "beam_size",
                "Beam size must be greater than 0",
            ));
        }
    }

    // Validate max tokens if specified
    if let Some(max_tokens) = options.max_tokens {
        if max_tokens == 0 {
            errors.push(ValidationError::new(
                "max_tokens",
                "Max tokens must be greater than 0",
            ));
        }
    }

    if errors.is_empty() {
        Ok(Vec::new())
    } else {
        Ok(errors)
    }
}

/// Convert TranscriptionOptions to TranscriptionConfig
pub fn options_to_config(options: TranscriptionOptions) -> TranscriptionConfig {
    debug!("Converting TranscriptionOptions to TranscriptionConfig");
    info!(
        "Converting TranscriptionOptions to TranscriptionConfig: {:?}",
        options
    );

    TranscriptionConfig {
        language: options.language,
        translate_to_english: options.translate_to_english.unwrap_or(false),
        include_timestamps: options.include_timestamps.unwrap_or(false),
        max_tokens: options.max_tokens,
        temperature: options.temperature.unwrap_or(0.0),
        use_beam_search: options.use_beam_search.unwrap_or(false),
        beam_size: options.beam_size,
        suppress_blank: options.suppress_blank.unwrap_or(true),
        word_timestamps: options.word_timestamps.unwrap_or(false),
    }
}

/// Transcription service using whisper-rs
pub struct TranscriptionService {
    context: WhisperContext,
    config: TranscriptionConfig,
}

impl std::fmt::Debug for TranscriptionService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TranscriptionService")
            .field("config", &self.config)
            .field("model_info", &self.model_info())
            .finish()
    }
}

impl TranscriptionService {
    /// Create a new transcription service with the given Whisper context and configuration
    ///
    /// # Arguments
    /// * `context` - The loaded Whisper context
    /// * `config` - Transcription configuration
    ///
    /// # Returns
    /// * `Result<Self, TranscriptionError>` - New transcription service on success
    pub fn new(
        context: WhisperContext,
        config: TranscriptionConfig,
    ) -> Result<Self, TranscriptionError> {
        debug!("Creating transcription service with config: {:?}", config);
        info!("Creating transcription service with config: {:?}", config);
        Ok(Self { context, config })
    }

    /// Perform transcription on audio data
    ///
    /// # Arguments
    /// * `audio_data` - Raw audio data bytes (16kHz mono PCM)
    ///
    /// # Returns
    /// * `Result<TranscriptionResult, TranscriptionError>` - Transcription result
    pub fn transcribe(&self, audio_data: &[u8]) -> Result<TranscriptionResult, TranscriptionError> {
        let start_time = std::time::Instant::now();

        debug!(
            "Starting transcription on {} bytes of audio data",
            audio_data.len()
        );
        info!(
            "Starting transcription on {} bytes of audio data",
            audio_data.len()
        );

        // Validate audio data
        if audio_data.is_empty() {
            debug!("Audio data validation failed: empty data");
            return Err(TranscriptionError::AudioDataError(
                "Audio data is empty".to_string(),
            ));
        }

        // Create full parameters for transcription
        debug!("Creating transcription parameters with beam search");
        let mut params = FullParams::new(SamplingStrategy::BeamSearch {
            beam_size: self.config.beam_size.unwrap_or(5),
            patience: 1.0,
        });

        // Set language if specified
        if let Some(ref lang) = self.config.language {
            debug!("Setting language to: {}", lang);
            info!("Setting language to: {}", lang);
            params.set_language(Some(lang.as_str()));
        } else {
            debug!("No language specified, will auto-detect");
            info!("No language specified, will auto-detect");
        }

        // Set translation to English if requested
        if self.config.translate_to_english {
            debug!("Translation to English enabled");
            info!("Translation to English enabled");
            params.set_translate(true);
        }

        // Set temperature
        debug!("Setting temperature to: {}", self.config.temperature);
        params.set_temperature(self.config.temperature);

        // Set token suppression
        debug!("Setting suppress_blank to: {}", self.config.suppress_blank);
        params.set_suppress_blank(self.config.suppress_blank);

        // Set word timestamps if enabled
        if self.config.word_timestamps {
            debug!("Word timestamps enabled");
            info!("Word timestamps enabled");
            params.set_no_timestamps(false);
        }

        // Set max tokens if specified
        if let Some(max_tokens) = self.config.max_tokens {
            debug!("Setting max tokens to: {}", max_tokens);
            info!("Setting max tokens to: {}", max_tokens);
            params.set_max_tokens(max_tokens as i32);
        }

        // Set number of threads (use system optimal if not specified)
        let num_threads = 4; // Default to 4 threads
        debug!("Using {} threads for transcription", num_threads);
        info!("Using {} threads for transcription", num_threads);
        params.set_n_threads(num_threads as i32);

        // Log the parameters
        debug!("Transcription parameters:");
        info!("Transcription parameters:");
        debug!("  Language: {:?}", self.config.language);
        info!("  Language: {:?}", self.config.language);
        debug!(
            "  Translate to English: {}",
            self.config.translate_to_english
        );
        info!(
            "  Translate to English: {}",
            self.config.translate_to_english
        );
        debug!("  Temperature: {}", self.config.temperature);
        info!("  Temperature: {}", self.config.temperature);
        debug!("  Beam search: {}", self.config.use_beam_search);
        info!("  Beam search: {}", self.config.use_beam_search);
        debug!("  Suppress blank: {}", self.config.suppress_blank);
        info!("  Suppress blank: {}", self.config.suppress_blank);
        debug!("  Word timestamps: {}", self.config.word_timestamps);
        info!("  Word timestamps: {}", self.config.word_timestamps);

        debug!("Converting audio data to f32 format");
        // Convert audio data to f32 (whisper-rs expects f32 samples)
        let audio_data_f32: Vec<f32> = audio_data
            .chunks_exact(2) // 16-bit samples are 2 bytes (little endian order)
            .map(|chunk| {
                if let [low, high] = chunk {
                    ((i16::from(*high) << 8) | i16::from(*low)) as f32 / 32768.0
                } else {
                    0.0 // Handle incomplete chunks
                }
            })
            .collect();
        debug!(
            "Converted {} bytes to {} f32 samples",
            audio_data.len(),
            audio_data_f32.len()
        );

        // Perform the transcription
        debug!("Creating Whisper state for transcription");
        let mut state = match self.context.create_state() {
            Ok(state) => {
                debug!("Whisper state created successfully");
                state
            }
            Err(e) => {
                error!("Failed to create Whisper state: {}", e);
                return Err(TranscriptionError::WhisperContextError(e.to_string()));
            }
        };

        debug!("Starting audio processing with Whisper");
        // Process the audio data
        match state.full(params, &audio_data_f32) {
            Ok(_) => {
                debug!("Whisper processing completed successfully");
                info!("Transcription completed successfully");

                // Extract the results
                debug!("Extracting transcription results");
                let result = self.extract_transcription_result(&state, start_time.elapsed())?;

                Ok(result)
            }
            Err(e) => {
                debug!("Whisper processing failed: {}", e);
                error!("Transcription failed: {}", e);
                Err(TranscriptionError::TranscriptionFailed(e.to_string()))
            }
        }
    }

    /// Extract transcription results from the Whisper state
    ///
    /// # Arguments
    /// * `state` - The Whisper state containing the results
    /// * `duration` - Time taken for transcription
    ///
    /// # Returns
    /// * `Result<TranscriptionResult, TranscriptionError>` - Extracted result
    fn extract_transcription_result(
        &self,
        state: &whisper_rs::WhisperState,
        duration: Duration,
    ) -> Result<TranscriptionResult, TranscriptionError> {
        debug!("Starting transcription result extraction");
        let mut text = String::new();
        let mut segments = Vec::new();
        // Get the language if available
        debug!("Extracting language from Whisper state");
        let lang_id = state.full_lang_id_from_state();
        let lang_code = match lang_id {
            0 => "en",
            1 => "zh",
            2 => "de",
            3 => "es",
            4 => "ru",
            5 => "ko",
            6 => "fr",
            7 => "ja",
            8 => "pt",
            9 => "tr",
            10 => "pl",
            11 => "ca",
            _ => "unknown",
        };
        debug!("Detected language ID: {} -> {}", lang_id, lang_code);
        info!("Detected language: {}", lang_code);

        // Get the number of segments
        let num_segments = state.full_n_segments();
        debug!("Transcription produced {} segments", num_segments);
        info!("Transcription produced {} segments", num_segments);

        // Extract segments if enabled
        debug!(
            "Extracting segments with timestamps: {}",
            self.config.include_timestamps || self.config.word_timestamps
        );
        if self.config.include_timestamps || self.config.word_timestamps {
            for i in 0..num_segments {
                debug!("Processing segment {}", i);
                if let Some(segment) = state.get_segment(i) {
                    match segment.to_str() {
                        Ok(segment_text) => {
                            let segment_text = segment_text.trim().to_string();
                            if !segment_text.is_empty() {
                                debug!("Segment {} text: \"{}\"", i, segment_text);
                                let trans_segment = TranscriptionSegment {
                                    start: segment.start_timestamp() as f32 / 100.0, // Convert from centiseconds to seconds
                                    end: segment.end_timestamp() as f32 / 100.0,
                                    text: segment_text.clone(),
                                    confidence: None, // API doesn't provide confidence in this version
                                };
                                segments.push(trans_segment.clone());
                                text.push_str(&segment_text);
                                text.push(' ');
                            }
                        }
                        Err(e) => {
                            warn!("Failed to get segment text {}: {}", i, e);
                        }
                    }
                } else {
                    warn!("Failed to get segment {}", i);
                }
            }
        } else {
            // Just get the full text without segments
            // Get the full text by concatenating all segments
            debug!("Extracting full text without segments");
            if num_segments > 0 {
                for i in 0..num_segments {
                    if let Some(segment) = state.get_segment(i) {
                        match segment.to_str() {
                            Ok(segment_text) => {
                                let segment_text = segment_text.trim().to_string();
                                if !segment_text.is_empty() {
                                    text.push_str(&segment_text);
                                    text.push(' ');
                                }
                            }
                            Err(e) => {
                                warn!("Failed to get segment text {}: {}", i, e);
                            }
                        }
                    } else {
                        warn!("Failed to get segment {}", i);
                    }
                }
            } else {
                warn!("No segments available for transcription");
            }
        }

        // Clean up the text
        debug!("Cleaning up transcribed text");
        text = text.trim().to_string();

        let duration_ms = duration.as_millis() as u64;

        debug!("Transcription completed in {} ms", duration_ms);
        info!("Transcription completed in {} ms", duration_ms);
        debug!("Transcribed text: \"{}\"", text);
        info!("Transcribed text: {}", text);

        Ok(TranscriptionResult {
            text,
            language: None,
            segments: if segments.is_empty() {
                None
            } else {
                Some(segments)
            },
            success: true,
            error: None,
            duration_ms: Some(duration_ms),
        })
    }

    /// Update the transcription configuration
    ///
    /// # Arguments
    /// * `config` - New configuration
    pub fn update_config(&mut self, config: TranscriptionConfig) {
        info!("Updating transcription configuration: {:?}", config);
        self.config = config;
    }

    /// Get the current transcription configuration
    ///
    /// # Returns
    /// * `&TranscriptionConfig` - Current configuration
    pub fn config(&self) -> &TranscriptionConfig {
        &self.config
    }

    /// Get information about the loaded model
    ///
    /// # Returns
    /// * `ModelInfo` - Model information
    pub fn model_info(&self) -> ModelInfo {
        ModelInfo {
            sampling_rate: 16000, // Default sampling rate for Whisper
            n_text_ctx: 448,      // Default text context size
            n_mels: 80,           // Default number of mel bins
            multilingual: true,   // Most Whisper models are multilingual
            has_encoder: true,    // Whisper has an encoder
            has_decoder: true,    // Whisper has a decoder
        }
    }
}

/// Information about the loaded model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Audio sampling rate
    pub sampling_rate: i32,
    /// Text context size
    pub n_text_ctx: i32,
    /// Number of mel frequency bins
    pub n_mels: i32,
    /// Whether the model is multilingual
    pub multilingual: bool,
    /// Whether the model has an encoder
    pub has_encoder: bool,
    /// Whether the model has a decoder
    pub has_decoder: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcription_config_default() {
        let config = TranscriptionConfig::default();

        assert!(config.language.is_none());
        assert!(!config.translate_to_english);
        assert!(!config.include_timestamps);
        assert!(config.max_tokens.is_none());
        assert_eq!(config.temperature, 0.0);
        assert!(!config.use_beam_search); // Note: This remains false to maintain backward compatibility
        assert!(config.beam_size.is_none()); // Note: This remains None to maintain backward compatibility
        assert!(config.suppress_blank);
        assert!(!config.word_timestamps);
    }

    #[test]
    fn test_transcription_config_custom() {
        let config = TranscriptionConfig {
            language: Some("en".to_string()),
            translate_to_english: true,
            include_timestamps: true,
            max_tokens: Some(100),
            temperature: 0.5,
            use_beam_search: true,
            beam_size: Some(5),
            suppress_blank: false,
            word_timestamps: true,
        };

        assert_eq!(config.language, Some("en".to_string()));
        assert!(config.translate_to_english);
        assert!(config.include_timestamps);
        assert_eq!(config.max_tokens, Some(100));
        assert_eq!(config.temperature, 0.5);
        assert!(config.use_beam_search);
        assert_eq!(config.beam_size, Some(5));
        assert!(!config.suppress_blank);
        assert!(config.word_timestamps);
    }

    #[test]
    fn test_transcription_result() {
        let result = TranscriptionResult {
            text: "Hello world".to_string(),
            language: Some("en".to_string()),
            segments: None,
            success: true,
            error: None,
            duration_ms: Some(1000),
        };

        assert_eq!(result.text, "Hello world");
        assert_eq!(result.language, Some("en".to_string()));
        assert!(result.segments.is_none());
        assert!(result.success);
        assert!(result.error.is_none());
        assert_eq!(result.duration_ms, Some(1000));
    }

    #[test]
    fn test_transcription_segment() {
        let segment = TranscriptionSegment {
            start: 0.0,
            end: 1.0,
            text: "Hello".to_string(),
            confidence: Some(0.95),
        };

        assert_eq!(segment.start, 0.0);
        assert_eq!(segment.end, 1.0);
        assert_eq!(segment.text, "Hello");
        assert_eq!(segment.confidence, Some(0.95));
    }

    #[test]
    fn test_model_info() {
        // This test would require a real Whisper context to run properly
        // For now, we just test the structure
        let info = ModelInfo {
            sampling_rate: 16000,
            n_text_ctx: 448,
            n_mels: 80,
            multilingual: true,
            has_encoder: true,
            has_decoder: true,
        };

        assert_eq!(info.sampling_rate, 16000);
        assert_eq!(info.n_text_ctx, 448);
        assert_eq!(info.n_mels, 80);
        assert!(info.multilingual);
        assert!(info.has_encoder);
        assert!(info.has_decoder);
    }

    // JSON parsing and validation tests
    #[test]
    fn test_transcription_request_base64() {
        let json = r#"
        {
            "audio_data": {
                "data": "SGVsbG8gV29ybGQ=",
                "format": "pcm"
            },
            "options": {
                "language": "en",
                "include_timestamps": true
            }
        }
        "#;

        let request: TranscriptionRequest = serde_json::from_str(json).unwrap();

        match request.audio_data {
            AudioDataFormat::Base64 { data, .. } => {
                assert_eq!(data, "SGVsbG8gV29ybGQ=");
            }
            _ => panic!("Expected base64 format"),
        }

        assert_eq!(request.options.unwrap().language, Some("en".to_string()));
    }

    #[test]
    fn test_transcription_request_binary() {
        let json = r#"
        {
            "audio_data": {
                "data": [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]
            }
        }
        "#;

        let request: TranscriptionRequest = serde_json::from_str(json).unwrap();

        match request.audio_data {
            AudioDataFormat::Binary { data, .. } => {
                assert_eq!(
                    data,
                    vec![72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]
                );
            }
            _ => panic!("Expected binary format"),
        }

        assert!(request.options.is_none());
    }

    #[test]
    fn test_transcription_request_minimal() {
        let json = r#"
        {
            "audio_data": {
                "data": "SGVsbG8gV29ybGQ="
            }
        }
        "#;

        let request: TranscriptionRequest = serde_json::from_str(json).unwrap();

        match request.audio_data {
            AudioDataFormat::Base64 { data, .. } => {
                assert_eq!(data, "SGVsbG8gV29ybGQ=");
            }
            _ => panic!("Expected base64 format"),
        }

        assert!(request.options.is_none());
    }

    #[test]
    fn test_transcription_request_invalid_json() {
        let json = r#"
        {
            "audio_data": {
                "data": "SGVsbG8gV29ybGQ="
            ,
            "options": {
                "language": "invalid_lang"
            }
        }
        "#;

        let result: Result<TranscriptionRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_transcription_request_missing_audio_data() {
        let json = r#"
        {
            "options": {
                "language": "en"
            }
        }
        "#;

        let result: Result<TranscriptionRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_transcription_options_serialization() {
        let options = TranscriptionOptions {
            language: Some("en".to_string()),
            translate_to_english: Some(true),
            include_timestamps: Some(false),
            max_tokens: Some(500),
            temperature: Some(0.7),
            use_beam_search: Some(true),
            beam_size: Some(10),
            suppress_blank: Some(false),
            word_timestamps: Some(true),
        };

        let json = serde_json::to_string(&options).unwrap();
        let deserialized: TranscriptionOptions = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.language, options.language);
        assert_eq!(
            deserialized.translate_to_english,
            options.translate_to_english
        );
        assert_eq!(deserialized.include_timestamps, options.include_timestamps);
        assert_eq!(deserialized.max_tokens, options.max_tokens);
        assert_eq!(deserialized.temperature, options.temperature);
        assert_eq!(deserialized.use_beam_search, options.use_beam_search);
        assert_eq!(deserialized.beam_size, options.beam_size);
        assert_eq!(deserialized.suppress_blank, options.suppress_blank);
        assert_eq!(deserialized.word_timestamps, options.word_timestamps);
    }

    #[test]
    fn test_transcription_options_default() {
        let options = TranscriptionOptions::default();

        assert!(options.language.is_none());
        assert_eq!(options.translate_to_english, Some(false));
        assert_eq!(options.include_timestamps, Some(true));
        assert!(options.max_tokens.is_none());
        assert_eq!(options.temperature, Some(0.0));
        assert_eq!(options.use_beam_search, Some(true)); // Updated to match new default
        assert_eq!(options.beam_size, Some(5)); // Updated to match new default
        assert_eq!(options.suppress_blank, Some(true));
        assert_eq!(options.word_timestamps, Some(false));
    }

    #[test]
    fn test_extract_audio_data_base64() {
        let request = TranscriptionRequest {
            audio_data: AudioDataFormat::Base64 {
                data: "SGVsbG8gV29ybGQ=".to_string(),
                _format: None,
            },
            options: None,
        };

        let result = extract_audio_data(&request).unwrap();
        assert_eq!(result, b"Hello World".to_vec());
    }

    #[test]
    fn test_extract_audio_data_binary() {
        let request = TranscriptionRequest {
            audio_data: AudioDataFormat::Binary {
                data: vec![72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100],
                _format: None,
            },
            options: None,
        };

        let result = extract_audio_data(&request).unwrap();
        assert_eq!(
            result,
            vec![72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]
        );
    }

    #[test]
    fn test_extract_audio_data_empty_binary() {
        let request = TranscriptionRequest {
            audio_data: AudioDataFormat::Binary {
                data: vec![],
                _format: None,
            },
            options: None,
        };

        let result = extract_audio_data(&request);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Binary audio data is empty"));
    }

    #[test]
    fn test_extract_audio_data_invalid_base64() {
        let request = TranscriptionRequest {
            audio_data: AudioDataFormat::Base64 {
                data: "invalid_base64!@#".to_string(),
                _format: None,
            },
            options: None,
        };

        let result = extract_audio_data(&request);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to decode base64"));
    }

    #[test]
    fn test_validate_transcription_options_valid() {
        let options = TranscriptionOptions {
            language: Some("en".to_string()),
            temperature: Some(0.5),
            beam_size: Some(5),
            max_tokens: Some(100),
            ..Default::default()
        };

        let result = validate_transcription_options(&options).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_validate_transcription_options_invalid_language() {
        let options = TranscriptionOptions {
            language: Some("invalid_lang".to_string()),
            ..Default::default()
        };

        let result = validate_transcription_options(&options).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].field, "language");
        assert!(result[0].message.contains("Invalid language code"));
    }

    #[test]
    fn test_validate_transcription_options_invalid_temperature() {
        let options = TranscriptionOptions {
            temperature: Some(2.0),
            ..Default::default()
        };

        let result = validate_transcription_options(&options).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].field, "temperature");
        assert!(result[0].message.contains("between 0.0 and 1.0"));
    }

    #[test]
    fn test_validate_transcription_options_invalid_beam_size() {
        let options = TranscriptionOptions {
            beam_size: Some(0),
            ..Default::default()
        };

        let result = validate_transcription_options(&options).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].field, "beam_size");
        assert!(result[0].message.contains("greater than 0"));
    }

    #[test]
    fn test_validate_transcription_options_invalid_max_tokens() {
        let options = TranscriptionOptions {
            max_tokens: Some(0),
            ..Default::default()
        };

        let result = validate_transcription_options(&options).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].field, "max_tokens");
        assert!(result[0].message.contains("greater than 0"));
    }

    #[test]
    fn test_validate_transcription_options_multiple_errors() {
        let options = TranscriptionOptions {
            language: Some("invalid_lang".to_string()),
            temperature: Some(2.0),
            beam_size: Some(0),
            max_tokens: Some(0),
            ..Default::default()
        };

        let result = validate_transcription_options(&options).unwrap();
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_options_to_config() {
        let options = TranscriptionOptions {
            language: Some("en".to_string()),
            translate_to_english: Some(true),
            include_timestamps: Some(false),
            max_tokens: Some(500),
            temperature: Some(0.7),
            use_beam_search: Some(true),
            beam_size: Some(10),
            suppress_blank: Some(false),
            word_timestamps: Some(true),
        };

        let config = options_to_config(options);

        assert_eq!(config.language, Some("en".to_string()));
        assert!(config.translate_to_english);
        assert!(!config.include_timestamps);
        assert_eq!(config.max_tokens, Some(500));
        assert_eq!(config.temperature, 0.7);
        assert!(config.use_beam_search);
        assert_eq!(config.beam_size, Some(10));
        assert!(!config.suppress_blank);
        assert!(config.word_timestamps);
    }

    #[test]
    fn test_update_config_from_options() {
        let base_config = TranscriptionConfig {
            language: Some("fr".to_string()),
            translate_to_english: false,
            include_timestamps: true,
            max_tokens: None,
            temperature: 0.0,
            use_beam_search: false,
            beam_size: None,
            suppress_blank: true,
            word_timestamps: false,
        };

        let options = TranscriptionOptions {
            language: Some("en".to_string()),
            translate_to_english: Some(true),
            include_timestamps: Some(false),
            max_tokens: Some(500),
            temperature: Some(0.7),
            use_beam_search: Some(true),
            beam_size: Some(10),
            suppress_blank: Some(false),
            word_timestamps: Some(true),
        };

        let updated_config = update_config_from_options(&base_config, &options);

        // Should update from options
        assert_eq!(updated_config.language, Some("en".to_string()));
        assert!(updated_config.translate_to_english);
        assert!(!updated_config.include_timestamps);
        assert_eq!(updated_config.max_tokens, Some(500));
        assert_eq!(updated_config.temperature, 0.7);
        assert!(updated_config.use_beam_search);
        assert_eq!(updated_config.beam_size, Some(10));
        assert!(!updated_config.suppress_blank);
        assert!(updated_config.word_timestamps);
    }

    #[test]
    fn test_json_error_display() {
        let error = JsonError::InvalidJson("test error".to_string());
        assert_eq!(format!("{}", error), "Invalid JSON: test error");

        let error = JsonError::MissingField("audio_data".to_string());
        assert_eq!(format!("{}", error), "Missing required field: audio_data");

        let error = JsonError::InvalidFieldValue("language".to_string(), "invalid".to_string());
        assert_eq!(
            format!("{}", error),
            "Invalid value for field 'language': invalid"
        );

        let error = JsonError::InvalidBase64("decode error".to_string());
        assert_eq!(
            format!("{}", error),
            "Invalid base64 encoding: decode error"
        );

        let error = JsonError::AudioDataError("empty data".to_string());
        assert_eq!(format!("{}", error), "Audio data error: empty data");
    }

    #[test]
    fn test_validation_error() {
        let error = ValidationError::new("field_name", "error message");
        assert_eq!(error.field, "field_name");
        assert_eq!(error.message, "error message");
    }
}
