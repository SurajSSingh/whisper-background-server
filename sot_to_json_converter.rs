//! SOT to JSON Converter
//! 
//! This utility provides tools for converting from the legacy SOT protocol to the new JSON interface.
//! It includes both a standalone converter and a compatibility layer for gradual migration.

use base64::engine::general_purpose;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

/// Audio data format for JSON interface
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

/// Transcription options for JSON interface
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

/// Complete transcription request for JSON interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionRequest {
    /// Audio data - can be base64 encoded string or binary data
    pub audio_data: AudioDataFormat,
    /// Transcription options
    pub options: Option<TranscriptionOptions>,
}

/// SOT marker detection and extraction
pub struct SotProcessor;

impl SotProcessor {
    /// Check if data contains SOT marker
    pub fn has_sot_marker(data: &[u8]) -> bool {
        data.windows(4).any(|window| window == b"\0SOT\0")
    }

    /// Extract audio data before SOT marker
    pub fn extract_audio_from_sot(data: &[u8]) -> Result<Vec<u8>, String> {
        // Find SOT marker position
        if let Some(position) = data.windows(4).position(|window| window == b"\0SOT\0") {
            // Return data before the SOT marker
            Ok(data[..position].to_vec())
        } else {
            Err("No SOT marker found in data".to_string())
        }
    }

    /// Convert SOT-based audio data to JSON format
    pub fn convert_sot_to_json(
        audio_data: &[u8],
        options: Option<TranscriptionOptions>,
    ) -> Result<TranscriptionRequest, String> {
        // Extract audio data before SOT marker
        let extracted_audio = Self::extract_audio_from_sot(audio_data)?;

        // Create base64-encoded audio data
        let base64_audio = general_purpose::STANDARD.encode(&extracted_audio);

        // Create JSON request
        let request = TranscriptionRequest {
            audio_data: AudioDataFormat::Base64 {
                data: base64_audio,
                _format: Some("wav".to_string()),
            },
            options,
        };

        Ok(request)
    }
}

/// Compatibility layer for gradual migration
pub struct CompatibilityLayer;

impl CompatibilityLayer {
    /// Detect input format (SOT or JSON)
    pub fn detect_input_format(data: &[u8]) -> InputFormat {
        // Check for SOT marker first
        if SotProcessor::has_sot_marker(data) {
            InputFormat::SOT
        } else {
            // Try to parse as JSON
            match serde_json::from_slice::<serde_json::Value>(data) {
                Ok(_) => InputFormat::JSON,
                Err(_) => InputFormat::Unknown,
            }
        }
    }

    /// Convert SOT data to JSON format
    pub fn convert_sot_to_json_request(
        data: &[u8],
        options: Option<TranscriptionOptions>,
    ) -> Result<TranscriptionRequest, String> {
        SotProcessor::convert_sot_to_json(data, options)
    }

    /// Convert JSON request to SOT-like binary data (for testing)
    pub fn convert_json_to_sot_like(
        request: &TranscriptionRequest,
    ) -> Result<Vec<u8>, String> {
        match &request.audio_data {
            AudioDataFormat::Base64 { data, .. } => {
                // Decode base64 and add SOT marker
                let audio_data = general_purpose::STANDARD
                    .decode(data)
                    .map_err(|e| format!("Failed to decode base64: {}", e))?;
                
                let mut result = Vec::with_capacity(audio_data.len() + 4);
                result.extend_from_slice(&audio_data);
                result.extend_from_slice(b"\0SOT\0");
                
                Ok(result)
            }
            AudioDataFormat::Binary { data, .. } => {
                // Use binary data and add SOT marker
                let mut result = Vec::with_capacity(data.len() + 4);
                result.extend_from_slice(data);
                result.extend_from_slice(b"\0SOT\0");
                
                Ok(result)
            }
        }
    }
}

/// Input format detection
#[derive(Debug, Clone, PartialEq)]
pub enum InputFormat {
    SOT,
    JSON,
    Unknown,
}

/// Command line arguments parser
#[derive(Debug, Clone)]
pub struct Args {
    pub input_file: Option<String>,
    pub output_file: Option<String>,
    pub output_format: OutputFormat,
    pub language: Option<String>,
    pub include_timestamps: bool,
    pub temperature: f32,
    pub verbose: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Json,
    SotLike,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            input_file: None,
            output_file: None,
            output_format: OutputFormat::Json,
            language: None,
            include_timestamps: true,
            temperature: 0.0,
            verbose: false,
        }
    }
}

impl Args {
    pub fn parse() -> Result<Self, String> {
        let mut args = Args::default();
        let mut cli_args = std::env::args().skip(1);

        while let Some(arg) = cli_args.next() {
            match arg.as_str() {
                "--input" | "-i" => {
                    args.input_file = Some(cli_args
                        .next()
                        .ok_or("--input requires a value")?);
                }
                "--output" | "-o" => {
                    args.output_file = Some(cli_args
                        .next()
                        .ok_or("--output requires a value")?);
                }
                "--format" | "-f" => {
                    let format = cli_args
                        .next()
                        .ok_or("--format requires a value")?;
                    args.output_format = match format {
                        "json" => OutputFormat::Json,
                        "sot" => OutputFormat::SotLike,
                        _ => return Err(format!("Invalid format: {}. Use 'json' or 'sot'", format)),
                    };
                }
                "--language" | "-l" => {
                    args.language = Some(cli_args
                        .next()
                        .ok_or("--language requires a value")?);
                }
                "--timestamps" | "-t" => {
                    args.include_timestamps = true;
                }
                "--no-timestamps" => {
                    args.include_timestamps = false;
                }
                "--temperature" => {
                    let temp_str = cli_args
                        .next()
                        .ok_or("--temperature requires a value")?;
                    args.temperature = temp_str
                        .parse()
                        .map_err(|_| "Invalid temperature value")?;
                }
                "--verbose" | "-v" => {
                    args.verbose = true;
                }
                "--help" | "-h" => {
                    Self::print_help();
                    std::process::exit(0);
                }
                _ => return Err(format!("Unknown argument: {}", arg)),
            }
        }

        Ok(args)
    }

    fn print_help() {
        println!("SOT to JSON Converter");
        println!("=====================");
        println!();
        println!("Convert audio data from SOT protocol to JSON interface.");
        println!();
        println!("USAGE:");
        println!("  sot_to_json_converter [OPTIONS]");
        println!();
        println!("OPTIONS:");
        println!("  -i, --input <file>       Input file (reads from stdin if not specified)");
        println!("  -o, --output <file>      Output file (writes to stdout if not specified)");
        println!("  -f, --format <format>    Output format: json, sot (default: json)");
        println!("  -l, --language <code>    Language code (e.g., en, es, fr)");
        println!("  -t, --timestamps         Include timestamps in output");
        println!("      --no-timestamps      Don't include timestamps");
        println!("      --temperature <num>  Temperature for sampling (0.0-1.0)");
        println!("  -v, --verbose            Enable verbose output");
        println!("  -h, --help               Show this help message");
        println!();
        println!("EXAMPLES:");
        println!("  # Convert SOT data to JSON from stdin to stdout");
        println!("  cat audio_data.sot | ./sot_to_json_converter");
        println!();
        println!("  # Convert file with specific options");
        println!("  ./sot_to_json_converter -i audio.sot -o audio.json -l en -t");
        println!();
        println!("  # Convert to SOT-like format for testing");
        println!("  ./sot_to_json_converter -f sot -i audio.json -o audio.sot");
    }
}

/// Main conversion logic
pub struct Converter;

impl Converter {
    /// Convert input data to specified format
    pub fn convert(
        input_data: &[u8],
        args: &Args,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let input_format = CompatibilityLayer::detect_input_format(input_data);

        if args.verbose {
            println!("Detected input format: {:?}", input_format);
        }

        let output_data = match input_format {
            InputFormat::SOT => {
                if args.verbose {
                    println!("Converting from SOT to JSON...");
                }

                let options = Some(TranscriptionOptions {
                    language: args.language.clone(),
                    include_timestamps: Some(args.include_timestamps),
                    temperature: Some(args.temperature),
                    ..Default::default()
                });

                let request = CompatibilityLayer::convert_sot_to_json_request(input_data, options)
                    .map_err(|e| format!("SOT conversion failed: {}", e))?;

                match args.output_format {
                    OutputFormat::Json => {
                        serde_json::to_vec(&request)
                            .map_err(|e| format!("JSON serialization failed: {}", e))?
                    }
                    OutputFormat::SotLike => {
                        CompatibilityLayer::convert_json_to_sot_like(&request)
                            .map_err(|e| format!("SOT-like conversion failed: {}", e))?
                    }
                }
            }
            InputFormat::JSON => {
                if args.verbose {
                    println!("Input is already JSON, converting to requested format...");
                }

                let request: TranscriptionRequest = serde_json::from_slice(input_data)
                    .map_err(|e| format!("JSON parsing failed: {}", e))?;

                match args.output_format {
                    OutputFormat::Json => {
                        // Just re-serialize (could add validation here)
                        serde_json::to_vec(&request)
                            .map_err(|e| format!("JSON serialization failed: {}", e))?
                    }
                    OutputFormat::SotLike => {
                        CompatibilityLayer::convert_json_to_sot_like(&request)
                            .map_err(|e| format!("SOT-like conversion failed: {}", e))?
                    }
                }
            }
            InputFormat::Unknown => {
                return Err("Unknown input format. Expected SOT data or JSON.".into());
            }
        };

        Ok(output_data)
    }

    /// Convert from file
    pub fn convert_file(
        input_path: &Path,
        output_path: &Path,
        args: &Args,
    ) -> Result<(), Box<dyn Error>> {
        let input_data = fs::read(input_path)
            .map_err(|e| format!("Failed to read input file: {}", e))?;

        let output_data = Self::convert(&input_data, args)?;

        fs::write(output_path, &output_data)
            .map_err(|e| format!("Failed to write output file: {}", e))?;

        if args.verbose {
            println!("Successfully converted {} to {}", input_path.display(), output_path.display());
        }

        Ok(())
    }
}

/// Main function
fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args = Args::parse()?;

    // Read input data
    let input_data = if let Some(input_file) = &args.input_file {
        fs::read(input_file)
            .map_err(|e| format!("Failed to read input file '{}': {}", input_file, e))?
    } else {
        // Read from stdin
        let mut buffer = Vec::new();
        io::stdin()
            .read_to_end(&mut buffer)
            .map_err(|e| format!("Failed to read from stdin: {}", e))?;
        buffer
    };

    // Convert data
    let output_data = Converter::convert(&input_data, &args)?;

    // Write output data
    if let Some(output_file) = &args.output_file {
        fs::write(output_file, &output_data)
            .map_err(|e| format!("Failed to write output file '{}': {}", output_file, e))?;
    } else {
        // Write to stdout
        io::stdout()
            .write_all(&output_data)
            .map_err(|e| format!("Failed to write to stdout: {}", e))?;
        io::stdout()
            .flush()
            .map_err(|e| format!("Failed to flush stdout: {}", e))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sot_marker_detection() {
        let data_with_sot = b"audio_data\0SOT\0";
        let data_without_sot = b"audio_data";
        
        assert!(SotProcessor::has_sot_marker(data_with_sot));
        assert!(!SotProcessor::has_sot_marker(data_without_sot));
    }

    #[test]
    fn test_audio_extraction_from_sot() {
        let data = b"some_audio_data\0SOT\0rest_of_data";
        let expected = b"some_audio_data";
        
        let result = SotProcessor::extract_audio_from_sot(data).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sot_to_json_conversion() {
        let audio_data = b"test_audio\0SOT\0";
        let options = Some(TranscriptionOptions {
            language: Some("en".to_string()),
            include_timestamps: Some(true),
            ..Default::default()
        });

        let request = SotProcessor::convert_sot_to_json(audio_data, options).unwrap();
        
        match request.audio_data {
            AudioDataFormat::Base64 { data, .. } => {
                assert!(!data.is_empty());
            }
            _ => panic!("Expected base64 format"),
        }
        
        assert_eq!(request.options.unwrap().language, Some("en".to_string()));
    }

    #[test]
    fn test_input_format_detection() {
        let sot_data = b"audio\0SOT\0";
        let json_data = br#"{ "audio_data": { "data": "test" } }"#;
        let unknown_data = b"unknown_data";

        assert_eq!(CompatibilityLayer::detect_input_format(sot_data), InputFormat::SOT);
        assert_eq!(CompatibilityLayer::detect_input_format(json_data), InputFormat::JSON);
        assert_eq!(CompatibilityLayer::detect_input_format(unknown_data), InputFormat::Unknown);
    }

    #[test]
    fn test_json_to_sot_like_conversion() {
        let request = TranscriptionRequest {
            audio_data: AudioDataFormat::Base64 {
                data: "dGVzdF9hdWRpby8=".to_string(),
                _format: None,
            },
            options: None,
        };

        let result = CompatibilityLayer::convert_json_to_sot_like(&request).unwrap();
        assert!(result.windows(4).any(|window| window == b"\0SOT\0"));
    }

    #[test]
    fn test_argument_parsing() {
        let test_args = vec![
            "program_name",
            "--input",
            "test.sot",
            "--output",
            "test.json",
            "--language",
            "en",
            "--timestamps",
            "--temperature",
            "0.5",
            "--verbose",
        ];

        let args = Args::parse_from(test_args.iter()).unwrap();
        
        assert_eq!(args.input_file, Some("test.sot".to_string()));
        assert_eq!(args.output_file, Some("test.json".to_string()));
        assert_eq!(args.language, Some("en".to_string()));
        assert!(args.include_timestamps);
        assert_eq!(args.temperature, 0.5);
        assert!(args.verbose);
    }

    #[test]
    fn test_full_conversion_cycle() {
        // Create test SOT data
        let sot_data = b"test_audio_data\0SOT\0";
        
        // Convert to JSON
        let options = Some(TranscriptionOptions {
            language: Some("en".to_string()),
            include_timestamps: Some(true),
            temperature: Some(0.0),
            ..Default::default()
        });

        let json_request = SotProcessor::convert_sot_to_json(sot_data, options).unwrap();
        let json_data = serde_json::to_vec(&json_request).unwrap();

        // Convert back to SOT-like
        let sot_like_data = CompatibilityLayer::convert_json_to_sot_like(&json_request).unwrap();

        // Verify SOT marker is present
        assert!(sot_like_data.windows(4).any(|window| window == b"\0SOT\0"));
    }
}