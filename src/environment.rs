use std::path::Path;

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

/// Parse command line arguments and return configuration
///
/// # Arguments
/// * `args` - Iterator over command line arguments
///
/// # Returns
/// * `Result<Config, String>` - Configuration on success, error message on failure
pub fn parse_arguments<I, S>(args: I) -> Result<Config, String>
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
pub fn validate_model_path(model_path: &str) -> Result<(), String> {
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
