use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use whisper_rs::{WhisperContext, WhisperContextParameters};

mod audio;
mod environment;
mod logging;
mod transcription;
use audio::{AudioBuffer, AudioProcessor};
use environment::{Config, parse_arguments};
use transcription::{TranscriptionConfig, TranscriptionService};

/// Structure to hold the loaded model and configuration
#[derive(Debug)]
pub struct ServerState {
    /// Configuration used to initialize the server
    pub config: Config,
    /// Transcription service (contains the Whisper context)
    pub transcription_service: TranscriptionService,
}

/// Information about the loaded model and server state
#[derive(Serialize, Deserialize)]
pub struct ServerInfo {
    /// Provider information
    pub provider: String,
    /// Model name (extracted from file path)
    pub model_name: String,
    /// Whisper-rs version
    pub version: String,
    /// Model attributes
    pub attributes: ModelAttributes,
    /// Current parameters
    pub parameters: ServerParameters,
}

/// Model attributes and capabilities
#[derive(Serialize, Deserialize)]
pub struct ModelAttributes {
    /// File size in bytes
    pub file_size: u64,
    /// Model type (based on filename)
    pub model_type: String,
    /// Whether GPU acceleration is available
    pub gpu_available: bool,
    /// Whether GPU acceleration is enabled
    pub gpu_enabled: bool,
}

/// Current server parameters
#[derive(Serialize, Deserialize)]
pub struct ServerParameters {
    /// Number of threads configured
    pub threads: Option<usize>,
    /// CPU-only mode enabled
    pub cpu_only: bool,
    /// Audio format (always 16kHz mono PCM)
    pub audio_format: String,
}

/// Initialize the Whisper model with the given configuration
///
/// # Arguments
/// * `config` - Server configuration
///
/// # Returns
/// * `Result<ServerState, String>` - Initialized server state on success, error message on failure
pub async fn initialize_server(config: Config) -> Result<ServerState, String> {
    info!("Initializing Whisper Background Server");
    debug!("Model path: {}", config.model_path);
    debug!("Threads: {:?}", config.threads);
    debug!("CPU only: {}", config.cpu_only);

    // Validate model path
    environment::validate_model_path(&config.model_path)?;

    // Load the Whisper model
    info!("Loading Whisper model from: {}", config.model_path);

    // Set up parameters based on config
    let mut params = WhisperContextParameters::new();
    if !config.cpu_only {
        params.use_gpu(true);
    }

    let context = match WhisperContext::new_with_params(&config.model_path, params) {
        Ok(ctx) => {
            info!("Model loaded successfully");
            ctx
        }
        Err(e) => {
            error!("Failed to load model: {}", e);
            return Err(format!("Failed to load model: {}", e));
        }
    };

    // Note: Thread configuration may need to be set through different methods
    // or may not be available in this version of whisper-rs
    if let Some(threads) = config.threads {
        info!(
            "Note: Thread count {} specified, but may need to be configured differently",
            threads
        );
    }

    // Create transcription configuration
    let transcription_config = TranscriptionConfig {
        language: None, // Auto-detect
        translate_to_english: false,
        include_timestamps: true,
        max_tokens: None,
        temperature: 0.0,
        use_beam_search: true, // Updated to match new default
        beam_size: Some(5),    // Updated to match new default
        suppress_blank: true,
        word_timestamps: false,
    };

    // Create transcription service
    let transcription_service = match TranscriptionService::new(context, transcription_config) {
        Ok(service) => {
            info!("Transcription service created successfully");
            service
        }
        Err(e) => {
            error!("Failed to create transcription service: {:?}", e);
            return Err(format!("Failed to create transcription service: {:?}", e));
        }
    };

    // Create server state
    let server_state = ServerState {
        config,
        transcription_service,
    };

    // Send server info to stdout
    if let Err(e) = send_server_info(&server_state) {
        error!("Failed to send server info to stdout: {}", e);
        return Err(format!("Failed to send server info: {}", e));
    }

    info!("Server initialization completed successfully");
    Ok(server_state)
}

/// Send server information to stdout as JSON
///
/// # Arguments
/// * `server_state` - The initialized server state
///
/// # Returns
/// * `Result<(), String>` - Ok if successful, error message if failed
fn send_server_info(server_state: &ServerState) -> Result<(), String> {
    let path = Path::new(&server_state.config.model_path);

    // Get file size
    let file_size = path
        .metadata()
        .map(|m| m.len())
        .map_err(|e| format!("Cannot read file metadata: {}", e))?;

    // Extract model name from file path
    let model_name = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Check if GPU is available and enabled (simplified check)
    let gpu_available = false; // TODO: Implement proper GPU availability check
    let gpu_enabled = !server_state.config.cpu_only && gpu_available;

    // Create server info
    let server_info = ServerInfo {
        provider: "whisper-rs".to_string(),
        model_name,
        version: env!("CARGO_PKG_VERSION").to_string(),
        attributes: ModelAttributes {
            file_size,
            model_type: "whisper".to_string(),
            gpu_available,
            gpu_enabled,
        },
        parameters: ServerParameters {
            threads: server_state.config.threads,
            cpu_only: server_state.config.cpu_only,
            audio_format: "16kHz mono PCM".to_string(),
        },
    };
    debug!("Sending server info");
    // Serialize to JSON and write to stdout
    match serde_json::to_string(&server_info) {
        Ok(json) => {
            println!("{}", json);
            io::stdout()
                .flush()
                .map_err(|e| format!("Failed to flush stdout: {}", e))?;
            Ok(())
        }
        Err(e) => Err(format!("Failed to serialize server info: {}", e)),
    }
}

/// Send transcription result to stdout as JSON
///
/// # Arguments
/// * `result` - The transcription result to format and send
///
/// # Returns
/// * `Result<(), String>` - Ok if successful, error message if failed
fn send_transcription_result_json(
    result: &transcription::TranscriptionResult,
) -> Result<(), String> {
    debug!("Formatting transcription result as JSON for output");

    // Create a structured output object that includes all relevant fields
    let output = TranscriptionOutput {
        text: result.text.clone(),
        language: result.language.clone(),
        segments: result.segments.clone(),
        success: result.success,
        error: result.error.clone(),
        duration_ms: result.duration_ms,
        timestamp: Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .to_string(),
        ),
    };

    // Serialize to JSON and write to stdout
    match serde_json::to_string(&output) {
        Ok(json) => {
            debug!("Successfully serialized transcription result to JSON");
            println!("{}", json);

            // Flush stdout to ensure the output is sent immediately
            match io::stdout().flush() {
                Ok(_) => {
                    debug!("Successfully flushed stdout after JSON output");
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to flush stdout after JSON output: {}", e);
                    Err(format!("Failed to flush stdout: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Failed to serialize transcription result to JSON: {}", e);
            Err(format!("Failed to serialize transcription result: {}", e))
        }
    }
}

/// Structured transcription output for JSON serialization
#[derive(Serialize, Deserialize)]
struct TranscriptionOutput {
    /// The transcribed text
    text: String,
    /// Language detected (if available)
    language: Option<String>,
    /// Segments with timestamps (if enabled)
    segments: Option<Vec<transcription::TranscriptionSegment>>,
    /// Whether the transcription was completed successfully
    success: bool,
    /// Error message if transcription failed
    error: Option<String>,
    /// Time taken for transcription in milliseconds
    duration_ms: Option<u64>,
    /// Timestamp when the result was generated (ISO 8601 format)
    timestamp: Option<String>,
}

/// Process JSON audio data from stdin using the async listener
///
/// # Arguments
/// * `server_state` - The initialized server state
///
/// # Returns
/// * `Result<(), String>` - Ok if successful, error message if failed
async fn process_audio_stream(server_state: &ServerState) -> Result<(), String> {
    debug!("Starting JSON audio processing from stdin");
    debug!(
        "JSON audio processing initialized with server state: {:?}",
        server_state
    );

    // Create audio buffer for JSON processing
    let mut audio_buffer = AudioBuffer::new();
    debug!("Audio buffer created for JSON processing");

    // Process JSON audio data as it arrives
    debug!("Starting JSON audio processing loop");
    loop {
        debug!("Reading JSON audio data from stdin");
        match audio::read_json_audio().await {
            Ok(Some(audio_data)) => {
                debug!("Received JSON audio data: {} bytes", audio_data.data.len());

                // Add audio data to buffer
                if let Err(e) = audio_buffer.process_audio(&audio_data) {
                    error!("Failed to process audio data: {}", e);
                    continue;
                }

                // Log buffer status
                let total_bytes = audio_buffer.total_bytes_received();
                debug!("Buffer contains {} bytes", total_bytes);

                // Check if buffer is ready and process audio data
                if audio_buffer.is_ready() {
                    debug!("Audio buffer ready for transcription");

                    // Take audio data for transcription
                    if let Some(audio_data) = audio_buffer.take_audio_data() {
                        debug!(
                            "Extracted {} bytes for transcription",
                            audio_data.data.len()
                        );

                        // Perform transcription using the transcription service
                        debug!("Starting transcription process");
                        match server_state
                            .transcription_service
                            .transcribe(&audio_data.data)
                        {
                            Ok(result) => {
                                debug!("Transcription completed successfully");
                                debug!("Transcribed text: {}", result.text);

                                if let Some(language) = &result.language {
                                    debug!("Detected language: {}", language);
                                }

                                if let Some(duration_ms) = result.duration_ms {
                                    debug!("Transcription took {} ms", duration_ms);
                                }

                                debug!("Formatting transcription result as JSON for output");
                                // Format and send result to stdout as JSON
                                match send_transcription_result_json(&result) {
                                    Ok(_) => {
                                        debug!(
                                            "Transcription result successfully sent to stdout as JSON"
                                        );
                                    }
                                    Err(e) => {
                                        error!(
                                            "Failed to send transcription result to stdout: {}",
                                            e
                                        );
                                        // Log error to stderr as fallback
                                        eprintln!("JSON output error: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Transcription failed: {}", e);
                                // Log error to stderr
                                eprintln!("Transcription error: {}", e);

                                // Send error result as JSON
                                debug!("Creating error result for JSON output");
                                let error_result = transcription::TranscriptionResult {
                                    text: String::new(),
                                    language: None,
                                    segments: None,
                                    success: false,
                                    error: Some(e.to_string()),
                                    duration_ms: None,
                                };

                                match send_transcription_result_json(&error_result) {
                                    Ok(_) => {
                                        debug!("Error result successfully sent to stdout as JSON");
                                    }
                                    Err(json_error) => {
                                        error!(
                                            "Failed to send error result to stdout: {}",
                                            json_error
                                        );
                                        eprintln!("JSON output error for result: {}", json_error);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(None) => {
                debug!("No more JSON audio data to process");
                break;
            }
            Err(e) => {
                error!("Error reading JSON audio data: {}", e);
                // Log error to stderr
                eprintln!("JSON audio data read error: {}", e);
                continue;
            }
        }
    }

    debug!("JSON audio processing completed");
    Ok(())
}

#[tokio::main]
async fn main() {
    // Initialize logging first
    logging::configure_logging();
    info!("Starting Whisper Background Server");

    // Parse command line arguments
    match parse_arguments(env::args()) {
        Ok(config) => {
            eprintln!("Configuration loaded successfully:");
            eprintln!("  Model path: {}", config.model_path);
            eprintln!("  Threads: {:?}", config.threads);
            eprintln!("  CPU only: {}", config.cpu_only);

            // Initialize server with configuration
            match initialize_server(config).await {
                Ok(server_state) => {
                    info!("Server initialized successfully, ready for audio processing");

                    // Start audio processing
                    if let Err(e) = process_audio_stream(&server_state).await {
                        error!("Audio processing failed: {}", e);
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                }
                Err(e) => {
                    error!("Failed to initialize server: {}", e);
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!(
                "Usage: whisper-background-server <model-path> [--threads <number>] [--cpu-only]"
            );
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // JSON interface tests
    #[test]
    fn test_server_info_serialization() {
        let server_info = ServerInfo {
            provider: "whisper-rs".to_string(),
            model_name: "test-model".to_string(),
            version: "1.0.0".to_string(),
            attributes: ModelAttributes {
                file_size: 1024,
                model_type: "whisper".to_string(),
                gpu_available: false,
                gpu_enabled: false,
            },
            parameters: ServerParameters {
                threads: Some(4),
                cpu_only: true,
                audio_format: "16kHz mono PCM".to_string(),
            },
        };

        let json = serde_json::to_string(&server_info).unwrap();
        let deserialized: ServerInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.provider, server_info.provider);
        assert_eq!(deserialized.model_name, server_info.model_name);
        assert_eq!(deserialized.version, server_info.version);
        assert_eq!(
            deserialized.attributes.file_size,
            server_info.attributes.file_size
        );
        assert_eq!(
            deserialized.attributes.model_type,
            server_info.attributes.model_type
        );
        assert_eq!(
            deserialized.attributes.gpu_available,
            server_info.attributes.gpu_available
        );
        assert_eq!(
            deserialized.attributes.gpu_enabled,
            server_info.attributes.gpu_enabled
        );
        assert_eq!(
            deserialized.parameters.threads,
            server_info.parameters.threads
        );
        assert_eq!(
            deserialized.parameters.cpu_only,
            server_info.parameters.cpu_only
        );
        assert_eq!(
            deserialized.parameters.audio_format,
            server_info.parameters.audio_format
        );
    }

    #[test]
    fn test_transcription_output_serialization() {
        let output = TranscriptionOutput {
            text: "Hello world".to_string(),
            language: Some("en".to_string()),
            segments: None,
            success: true,
            error: None,
            duration_ms: Some(1000),
            timestamp: Some("1234567890".to_string()),
        };

        let json = serde_json::to_string(&output).unwrap();
        let deserialized: TranscriptionOutput = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.text, output.text);
        assert_eq!(deserialized.language, output.language);
        assert_eq!(deserialized.segments, output.segments);
        assert_eq!(deserialized.success, output.success);
        assert_eq!(deserialized.error, output.error);
        assert_eq!(deserialized.duration_ms, output.duration_ms);
        assert_eq!(deserialized.timestamp, output.timestamp);
    }

    #[test]
    fn test_transcription_output_with_segments() {
        let segments = vec![
            transcription::TranscriptionSegment {
                start: 0.0,
                end: 1.0,
                text: "Hello".to_string(),
                confidence: Some(0.95),
            },
            transcription::TranscriptionSegment {
                start: 1.0,
                end: 2.0,
                text: "world".to_string(),
                confidence: Some(0.90),
            },
        ];

        let output = TranscriptionOutput {
            text: "Hello world".to_string(),
            language: Some("en".to_string()),
            segments: Some(segments.clone()),
            success: true,
            error: None,
            duration_ms: Some(1000),
            timestamp: Some("1234567890".to_string()),
        };

        let json = serde_json::to_string(&output).unwrap();
        let deserialized: TranscriptionOutput = serde_json::from_str(&json).unwrap();

        if let Some(deserialized_segments) = deserialized.segments {
            assert_eq!(deserialized_segments.len(), segments.len());
            assert_eq!(deserialized_segments[0].text, segments[0].text);
            assert_eq!(deserialized_segments[1].text, segments[1].text);
        } else {
            panic!("Expected segments but got None");
        }
    }

    #[test]
    fn test_transcription_output_error_case() {
        let output = TranscriptionOutput {
            text: String::new(),
            language: None,
            segments: None,
            success: false,
            error: Some("Transcription failed".to_string()),
            duration_ms: None,
            timestamp: Some("1234567890".to_string()),
        };

        let json = serde_json::to_string(&output).unwrap();
        let deserialized: TranscriptionOutput = serde_json::from_str(&json).unwrap();

        assert!(!deserialized.success);
        assert_eq!(deserialized.error, Some("Transcription failed".to_string()));
        assert_eq!(deserialized.text, String::new());
    }

    #[test]
    fn test_model_attributes_serialization() {
        let attributes = ModelAttributes {
            file_size: 2048,
            model_type: "base".to_string(),
            gpu_available: true,
            gpu_enabled: false,
        };

        let json = serde_json::to_string(&attributes).unwrap();
        let deserialized: ModelAttributes = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.file_size, attributes.file_size);
        assert_eq!(deserialized.model_type, attributes.model_type);
        assert_eq!(deserialized.gpu_available, attributes.gpu_available);
        assert_eq!(deserialized.gpu_enabled, attributes.gpu_enabled);
    }

    #[test]
    fn test_server_parameters_serialization() {
        let parameters = ServerParameters {
            threads: Some(8),
            cpu_only: false,
            audio_format: "16kHz mono PCM".to_string(),
        };

        let json = serde_json::to_string(&parameters).unwrap();
        let deserialized: ServerParameters = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.threads, parameters.threads);
        assert_eq!(deserialized.cpu_only, parameters.cpu_only);
        assert_eq!(deserialized.audio_format, parameters.audio_format);
    }

    #[test]
    fn test_server_parameters_default() {
        let parameters = ServerParameters {
            threads: None,
            cpu_only: false,
            audio_format: "16kHz mono PCM".to_string(),
        };

        let json = serde_json::to_string(&parameters).unwrap();
        let deserialized: ServerParameters = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.threads, None);
        assert_eq!(deserialized.cpu_only, false);
        assert_eq!(deserialized.audio_format, "16kHz mono PCM".to_string());
    }
}
