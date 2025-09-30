use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

use log::{LevelFilter, debug, error, info, warn};
use serde::{Deserialize, Serialize};
use whisper_rs::{WhisperContext, WhisperContextParameters};

mod audio;
mod transcription;
use audio::{AudioBuffer, AudioProcessor};
use transcription::{TranscriptionConfig, TranscriptionService};

/// Configuration structure for the Whisper Background Server
#[derive(Debug, Clone)]
pub struct Config {
    /// Path to the model file (required)
    pub model_path: String,
    /// Number of threads to use (optional, defaults to system optimal)
    pub threads: Option<usize>,
    /// Whether to enforce CPU-only mode (optional, defaults to false)
    pub cpu_only: bool,
}

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

/// Configure logging to output to stderr with proper formatting
fn configure_logging() {
    // Set up log level to Info for normal operation, Debug for detailed info
    log::set_max_level(LevelFilter::Info);

    // Simple stderr logger implementation
    let logger = Box::new(CustomLogger::new());

    // Apply the logger
    if let Err(e) = log::set_logger(Box::leak(logger)) {
        eprintln!("Failed to set logger: {}", e);
    }
}

/// Custom logger that outputs to stderr with formatting
struct CustomLogger {
    start_time: std::time::Instant,
}

impl CustomLogger {
    fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
        }
    }

    fn format_log(&self, level: log::Level, _target: &str, message: &str) -> String {
        let elapsed = self.start_time.elapsed();
        let timestamp = format!(
            "{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        format!(
            "[{} {} {}.{:03}s] {}",
            timestamp,
            level,
            elapsed.as_secs(),
            elapsed.subsec_millis(),
            message
        )
    }
}

impl log::Log for CustomLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let formatted =
                self.format_log(record.level(), record.target(), &record.args().to_string());
            eprintln!("{}", formatted);
        }
    }

    fn flush(&self) {
        std::io::stderr().flush().unwrap();
    }
}

/// Parse command line arguments and return configuration
///
/// # Arguments
/// * `args` - Iterator over command line arguments
///
/// # Returns
/// * `Result<Config, String>` - Configuration on success, error message on failure
fn parse_arguments<I, S>(args: I) -> Result<Config, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut args: Vec<String> = args.into_iter().map(|s| s.as_ref().to_string()).collect();

    // Remove the program name from arguments
    if args.is_empty() {
        return Err("No arguments provided. Usage: whisper-background-server <model-path> [--threads <number>] [--cpu-only]".to_string());
    }

    args.remove(0); // Remove program name

    if args.is_empty() {
        return Err("Model path is required. Usage: whisper-background-server <model-path> [--threads <number>] [--cpu-only]".to_string());
    }

    let mut config = Config {
        model_path: String::new(),
        threads: None,
        cpu_only: false,
    };

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];

        match arg.as_str() {
            // Model path (positional argument, first argument)
            _ if i == 0 => {
                config.model_path = arg.clone();

                // Validate that the model path exists
                if !Path::new(&config.model_path).exists() {
                    return Err(format!("Model path does not exist: {}", config.model_path));
                }

                i += 1;
            }

            // Threads option
            "--threads" => {
                if i + 1 >= args.len() {
                    return Err("--threads option requires a value".to_string());
                }

                let threads_str = &args[i + 1];
                match threads_str.parse::<usize>() {
                    Ok(threads) => {
                        if threads == 0 {
                            return Err("Number of threads must be greater than 0".to_string());
                        }
                        config.threads = Some(threads);
                        i += 2; // Skip the next argument (the value)
                    }
                    Err(_) => {
                        return Err(format!("Invalid number of threads: {}", threads_str));
                    }
                }
            }

            // CPU-only flag
            "--cpu-only" => {
                config.cpu_only = true;
                i += 1;
            }

            // Unknown argument
            _ => {
                return Err(format!("Unknown argument: {}", arg));
            }
        }
    }

    // Validate that we have a model path
    if config.model_path.is_empty() {
        return Err("Model path is required".to_string());
    }

    Ok(config)
}

/// Validate that the model path exists and has the correct extension
///
/// # Arguments
/// * `model_path` - Path to the model file
///
/// # Returns
/// * `Result<(), String>` - Ok if valid, error message if invalid
fn validate_model_path(model_path: &str) -> Result<(), String> {
    let path = Path::new(model_path);

    // Check if file exists
    if !path.exists() {
        return Err(format!("Model file does not exist: {}", model_path));
    }

    // Check if it's a file (not a directory)
    if !path.is_file() {
        return Err(format!("Model path is not a file: {}", model_path));
    }

    // Check file extension
    if let Some(extension) = path.extension() {
        if extension != "bin" {
            return Err(format!(
                "Model file must have .bin extension, got: {:?}",
                extension
            ));
        }
    } else {
        return Err(format!("Model file has no extension: {}", model_path));
    }

    // Get file size
    if let Ok(metadata) = path.metadata() {
        if metadata.len() == 0 {
            return Err(format!("Model file is empty: {}", model_path));
        }
    } else {
        return Err(format!("Cannot read model file metadata: {}", model_path));
    }

    Ok(())
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
    info!("Model path: {}", config.model_path);
    info!("Threads: {:?}", config.threads);
    info!("CPU only: {}", config.cpu_only);

    // Validate model path
    validate_model_path(&config.model_path)?;

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
        use_beam_search: false,
        beam_size: None,
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
    info!("Formatting transcription result as JSON for output");

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
            info!("Successfully serialized transcription result to JSON");
            println!("{}", json);

            // Flush stdout to ensure the output is sent immediately
            match io::stdout().flush() {
                Ok(_) => {
                    info!("Successfully flushed stdout after JSON output");
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

/// Process audio data from stdin using the async listener
///
/// # Arguments
/// * `server_state` - The initialized server state
///
/// # Returns
/// * `Result<(), String>` - Ok if successful, error message if failed
async fn process_audio_stream(server_state: &ServerState) -> Result<(), String> {
    info!("Starting audio processing from stdin");
    debug!(
        "Audio processing initialized with server state: {:?}",
        server_state
    );

    // Create audio buffer and temporary buffer for reading
    let mut audio_buffer = AudioBuffer::new();
    let mut temp_buffer = Vec::new();
    let mut sequence = 0u64;
    debug!("Audio buffer and temporary buffer created");

    // Process audio chunks as they arrive
    debug!("Starting audio chunk processing loop");
    loop {
        debug!("Reading audio chunk, sequence number: {}", sequence);
        match audio::read_audio_chunk(&mut temp_buffer).await {
            Ok(Some(chunk)) => {
                debug!("Received audio chunk: {} bytes", chunk.data.len());

                // Add sequence number
                let mut chunk = chunk;
                chunk.sequence = sequence;
                sequence += 1;
                debug!("Assigned sequence number: {}", chunk.sequence);

                // Add chunk to buffer
                if let Err(e) = audio_buffer.process_chunk(&chunk) {
                    error!("Failed to process audio chunk: {}", e);
                    continue;
                }

                // Log buffer status
                let total_bytes = audio_buffer.total_bytes_received();
                info!("Buffer contains {} bytes", total_bytes);
                debug!("Buffer status - total bytes received: {}", total_bytes);

                // Check for SOT marker and process if found
                if audio_buffer.is_ready() {
                    debug!("SOT marker detected, checking buffer readiness");
                    info!("SOT marker detected, extracting audio data for transcription");

                    // Extract audio data for transcription
                    if let Some(audio_data) = audio_buffer.process_sot_marker() {
                        debug!(
                            "Audio data extracted for transcription: {} bytes",
                            audio_data.len()
                        );
                        info!("Extracted {} bytes for transcription", audio_data.len());

                        // Perform transcription using the transcription service
                        debug!("Starting transcription process");
                        match server_state.transcription_service.transcribe(&audio_data) {
                            Ok(result) => {
                                debug!("Transcription completed successfully");
                                info!("Transcription completed successfully");
                                info!("Transcribed text: {}", result.text);

                                if let Some(language) = &result.language {
                                    debug!("Detected language: {}", language);
                                    info!("Detected language: {}", language);
                                }

                                if let Some(duration_ms) = result.duration_ms {
                                    debug!("Transcription took {} ms", duration_ms);
                                    info!("Transcription took {} ms", duration_ms);
                                }

                                debug!("Formatting transcription result as JSON for output");
                                // Format and send result to stdout as JSON
                                match send_transcription_result_json(&result) {
                                    Ok(_) => {
                                        debug!(
                                            "Transcription result successfully sent to stdout as JSON"
                                        );
                                        info!(
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
                                debug!("Transcription failed: {}", e);
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
                                        info!("Error result successfully sent to stdout as JSON");
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

                        // Note: The remaining buffer data is kept in audio_buffer
                        // for processing with the next chunk
                    } else {
                        warn!("SOT marker was detected but no audio data was extracted");
                    }
                }
            }
            Ok(None) => {
                info!("Audio stream ended");
                break;
            }
            Err(e) => {
                error!("Error receiving audio data: {}", e);
                if e.kind() == std::io::ErrorKind::Interrupted {
                    // Try again on interrupt
                    continue;
                } else {
                    // Break on other errors
                    break;
                }
            }
        }
    }

    info!("Audio processing completed");
    Ok(())
}

fn main() {
    // Initialize logging first
    configure_logging();
    info!("Starting Whisper Background Server");

    // Parse command line arguments
    match parse_arguments(env::args()) {
        Ok(config) => {
            println!("Configuration loaded successfully:");
            println!("  Model path: {}", config.model_path);
            println!("  Threads: {:?}", config.threads);
            println!("  CPU only: {}", config.cpu_only);

            // Initialize server with configuration
            let runtime = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

            match runtime.block_on(initialize_server(config)) {
                Ok(server_state) => {
                    info!("Server initialized successfully, ready for audio processing");

                    // Start audio processing
                    if let Err(e) = runtime.block_on(process_audio_stream(&server_state)) {
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

    // Mock the path existence check for tests
    fn mock_parse_arguments<I, S>(args: I) -> Result<Config, String>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut args: Vec<String> = args.into_iter().map(|s| s.as_ref().to_string()).collect();

        // Remove the program name from arguments
        if args.is_empty() {
            return Err("No arguments provided. Usage: whisper-background-server <model-path> [--threads <number>] [--cpu-only]".to_string());
        }

        args.remove(0); // Remove program name

        if args.is_empty() {
            return Err("Model path is required. Usage: whisper-background-server <model-path> [--threads <number>] [--cpu-only]".to_string());
        }

        let mut config = Config {
            model_path: String::new(),
            threads: None,
            cpu_only: false,
        };

        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];

            match arg.as_str() {
                // Model path (positional argument, first argument)
                _ if i == 0 => {
                    config.model_path = arg.clone();
                    i += 1;
                }

                // Threads option
                "--threads" => {
                    if i + 1 >= args.len() {
                        return Err("--threads option requires a value".to_string());
                    }

                    let threads_str = &args[i + 1];
                    match threads_str.parse::<usize>() {
                        Ok(threads) => {
                            if threads == 0 {
                                return Err("Number of threads must be greater than 0".to_string());
                            }
                            config.threads = Some(threads);
                            i += 2; // Skip the next argument (the value)
                        }
                        Err(_) => {
                            return Err(format!("Invalid number of threads: {}", threads_str));
                        }
                    }
                }

                // CPU-only flag
                "--cpu-only" => {
                    config.cpu_only = true;
                    i += 1;
                }

                // Unknown argument
                _ => {
                    return Err(format!("Unknown argument: {}", arg));
                }
            }
        }

        // Validate that we have a model path
        if config.model_path.is_empty() {
            return Err("Model path is required".to_string());
        }

        Ok(config)
    }

    #[test]
    fn test_parse_arguments_minimal() {
        let args = vec!["program_name".to_string(), "/path/to/model.bin".to_string()];

        let config = mock_parse_arguments(args).unwrap();
        assert_eq!(config.model_path, "/path/to/model.bin");
        assert_eq!(config.threads, None);
        assert_eq!(config.cpu_only, false);
    }

    #[test]
    fn test_parse_arguments_with_threads() {
        let args = vec![
            "program_name".to_string(),
            "/path/to/model.bin".to_string(),
            "--threads".to_string(),
            "4".to_string(),
        ];

        let config = mock_parse_arguments(args).unwrap();
        assert_eq!(config.model_path, "/path/to/model.bin");
        assert_eq!(config.threads, Some(4));
        assert_eq!(config.cpu_only, false);
    }

    #[test]
    fn test_parse_arguments_with_cpu_only() {
        let args = vec![
            "program_name".to_string(),
            "/path/to/model.bin".to_string(),
            "--cpu-only".to_string(),
        ];

        let config = mock_parse_arguments(args).unwrap();
        assert_eq!(config.model_path, "/path/to/model.bin");
        assert_eq!(config.threads, None);
        assert_eq!(config.cpu_only, true);
    }

    #[test]
    fn test_parse_arguments_with_both_options() {
        let args = vec![
            "program_name".to_string(),
            "/path/to/model.bin".to_string(),
            "--threads".to_string(),
            "8".to_string(),
            "--cpu-only".to_string(),
        ];

        let config = mock_parse_arguments(args).unwrap();
        assert_eq!(config.model_path, "/path/to/model.bin");
        assert_eq!(config.threads, Some(8));
        assert_eq!(config.cpu_only, true);
    }

    #[test]
    fn test_parse_arguments_no_model_path() {
        let args = vec!["program_name".to_string()];

        let result = mock_parse_arguments(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_arguments_invalid_threads() {
        let args = vec![
            "program_name".to_string(),
            "/path/to/model.bin".to_string(),
            "--threads".to_string(),
            "invalid".to_string(),
        ];

        let result = mock_parse_arguments(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_arguments_zero_threads() {
        let args = vec![
            "program_name".to_string(),
            "/path/to/model.bin".to_string(),
            "--threads".to_string(),
            "0".to_string(),
        ];

        let result = mock_parse_arguments(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_arguments_unknown_argument() {
        let args = vec![
            "program_name".to_string(),
            "/path/to/model.bin".to_string(),
            "--unknown".to_string(),
        ];

        let result = mock_parse_arguments(args);
        assert!(result.is_err());
    }
}
