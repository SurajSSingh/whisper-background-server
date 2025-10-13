# Whisper Background Server

A Rust-based transcription server that runs Whisper locally in the background, providing audio transcription services through a structured JSON interface.

## Overview

The Whisper Background Server is designed to run Whisper models locally on a user's computer and provide transcription services to other applications through a robust JSON-based interface. It abstracts away the complexity of running a transcription service locally and provides a clean, structured API for audio processing.

## Features

- **Local Whisper Processing**: Run Whisper models locally without external dependencies
- **JSON Interface**: Robust, structured JSON input/output for easy integration
- **Multiple Audio Formats**: Support for both base64-encoded and binary audio data
- **Configurable Transcription**: Comprehensive options for language, timestamps, temperature, and more
- **Error Handling**: Structured error responses with detailed information
- **Async Processing**: Built on Tokio for efficient async operations
- **Comprehensive Logging**: All logging goes to stderr to prevent interference with JSON output

## Installation

### Prerequisites

- Rust 1.90.0 or later
- A Whisper model file (`.bin` format)

### Build from Source

```bash
git clone https://github.com/your-repo/whisper-background-server.git
cd whisper-background-server
cargo build --release
```

### Download a Whisper Model

Download a Whisper model from [Hugging Face](https://huggingface.co/ggerganov/whisper.cpp) or use the provided models:

```bash
# Example: Download base English model
wget https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin
```

## Usage

### Basic Usage

```bash
./target/release/whisper-background-server large_files/ggml-base.en.bin
```

### With Options

```bash
./target/release/whisper-background-server large_files/ggml-base.en.bin --threads 4 --cpu-only
```

### Command Line Options

| Option | Description | Required |
|--------|-------------|----------|
| `<model-path>` | Path to the Whisper model file (`.bin`) | Yes |
| `--threads <number>` | Number of threads to use for processing | No |
| `--cpu-only` | Force CPU-only processing (disable GPU acceleration) | No |

## JSON Interface

The server communicates via JSON payloads on stdin and stdout. All logging is sent to stderr to prevent interference with JSON parsing.

### Server Initialization

When the server starts successfully, it outputs server information as JSON:

```json
{
  "provider": "whisper-rs",
  "model_name": "ggml-base.en",
  "version": "0.1.0",
  "attributes": {
    "file_size": 1415576576,
    "model_type": "whisper",
    "gpu_available": false,
    "gpu_enabled": false
  },
  "parameters": {
    "threads": null,
    "cpu_only": false,
    "audio_format": "16kHz mono PCM"
  }
}
```

### Transcription Request Format

Send transcription requests as JSON objects to stdin:

```json
{
  "audio_data": {
    "data": "base64_encoded_audio_string_or_binary_data"
  },
  "options": {
    "language": "en",
    "include_timestamps": true,
    "temperature": 0.0,
    "use_beam_search": false
  }
}
```

### Audio Data Formats

The server supports two audio data formats:

#### 1. Base64-Encoded Audio

```json
{
  "audio_data": {
    "data": "SGVsbG8gV29ybGQhISE=",
    "format": "pcm"
  }
}
```

#### 2. Binary Audio Data

```json
{
  "audio_data": {
    "data": [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]
  }
}
```

### Transcription Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `language` | string | `"auto"` | Language code (e.g., "en", "es", "fr", "zh", "de", "ru", "ko", "ja", "pt", "tr", "pl", "ca") |
| `translate_to_english` | boolean | `false` | Translate text to English (for multilingual models) |
| `include_timestamps` | boolean | `true` | Include timestamp segments in output |
| `max_tokens` | integer | `null` | Maximum number of tokens to generate |
| `temperature` | float | `0.0` | Temperature for sampling (0.0 to 1.0) |
| `use_beam_search` | boolean | `false` | Use beam search decoding |
| `beam_size` | integer | `null` | Number of beams for beam search (requires `use_beam_search: true`) |
| `suppress_blank` | boolean | `true` | Suppress blank tokens |
| `word_timestamps` | boolean | `false` | Enable word-level timestamps |

### Transcription Response Format

The server returns transcription results as JSON objects:

```json
{
  "text": "Hello world!",
  "language": "en",
  "segments": [
    {
      "start": 0.0,
      "end": 1.2,
      "text": "Hello world!",
      "confidence": null
    }
  ],
  "success": true,
  "error": null,
  "duration_ms": 245,
  "timestamp": "1640995200"
}
```

### Error Response Format

When errors occur, the server returns structured error responses:

```json
{
  "text": "",
  "language": null,
  "segments": null,
  "success": false,
  "error": "Failed to decode base64 audio data: Invalid input",
  "duration_ms": null,
  "timestamp": "1640995200"
}
```

## JSON Schema Reference

### TranscriptionRequest Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["audio_data"],
  "properties": {
    "audio_data": {
      "oneOf": [
        {
          "type": "object",
          "required": ["data"],
          "properties": {
            "data": {
              "type": "string",
              "description": "Base64 encoded audio data"
            },
            "format": {
              "type": "string",
              "description": "Format hint (optional)"
            }
          }
        },
        {
          "type": "object",
          "required": ["data"],
          "properties": {
            "data": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0,
                "maximum": 255
              },
              "description": "Binary audio data as byte array"
            },
            "format": {
              "type": "string",
              "description": "Format hint (optional)"
            }
          }
        }
      ]
    },
    "options": {
      "type": "object",
      "properties": {
        "language": {
          "type": "string",
          "enum": ["en", "auto", "zh", "de", "es", "ru", "ko", "fr", "ja", "pt", "tr", "pl", "ca"]
        },
        "translate_to_english": {
          "type": "boolean"
        },
        "include_timestamps": {
          "type": "boolean"
        },
        "max_tokens": {
          "type": "integer",
          "minimum": 1
        },
        "temperature": {
          "type": "number",
          "minimum": 0.0,
          "maximum": 1.0
        },
        "use_beam_search": {
          "type": "boolean"
        },
        "beam_size": {
          "type": "integer",
          "minimum": 1
        },
        "suppress_blank": {
          "type": "boolean"
        },
        "word_timestamps": {
          "type": "boolean"
        }
      }
    }
  }
}
```

### TranscriptionResult Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["text", "success", "error", "timestamp"],
  "properties": {
    "text": {
      "type": "string",
      "description": "The transcribed text"
    },
    "language": {
      "type": ["string", "null"],
      "description": "Detected language code"
    },
    "segments": {
      "type": ["array", "null"],
      "items": {
        "type": "object",
        "properties": {
          "start": {
            "type": "number",
            "description": "Start time in seconds"
          },
          "end": {
            "type": "number",
            "description": "End time in seconds"
          },
          "text": {
            "type": "string",
            "description": "Text content of the segment"
          },
          "confidence": {
            "type": ["number", "null"],
            "description": "Confidence score (0.0 to 1.0)"
          }
        },
        "required": ["start", "end", "text"]
      },
      "description": "Segments with timestamps"
    },
    "success": {
      "type": "boolean",
      "description": "Whether the transcription was completed successfully"
    },
    "error": {
      "type": ["string", "null"],
      "description": "Error message if transcription failed"
    },
    "duration_ms": {
      "type": ["integer", "null"],
      "description": "Time taken for transcription in milliseconds"
    },
    "timestamp": {
      "type": "string",
      "description": "Timestamp when the result was generated"
    }
  }
}
```

## Examples

### Example 1: Basic Transcription

**Request:**
```json
{
  "audio_data": {
    "data": "SGVsbG8gV29ybGQhISE="
  }
}
```

**Response:**
```json
{
  "text": "Hello World!!!",
  "language": "en",
  "segments": [
    {
      "start": 0.0,
      "end": 1.2,
      "text": "Hello World!!!",
      "confidence": null
    }
  ],
  "success": true,
  "error": null,
  "duration_ms": 156,
  "timestamp": "1640995200"
}
```

### Example 2: Transcription with Language and Timestamps

**Request:**
```json
{
  "audio_data": {
    "data": "SGVsbG8gV29ybGQhISE="
  },
  "options": {
    "language": "en",
    "include_timestamps": true,
    "temperature": 0.2
  }
}
```

**Response:**
```json
{
  "text": "Hello World!!!",
  "language": "en",
  "segments": [
    {
      "start": 0.0,
      "end": 1.2,
      "text": "Hello World!!!",
      "confidence": null
    }
  ],
  "success": true,
  "error": null,
  "duration_ms": 178,
  "timestamp": "1640995200"
}
```

### Example 3: Binary Audio Data

**Request:**
```json
{
  "audio_data": {
    "data": [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]
  },
  "options": {
    "language": "en",
    "include_timestamps": false
  }
}
```

**Response:**
```json
{
  "text": "Hello World!",
  "language": "en",
  "segments": null,
  "success": true,
  "error": null,
  "duration_ms": 134,
  "timestamp": "1640995200"
}
```

### Example 4: Multilingual Translation

**Request:**
```json
{
  "audio_data": {
    "data": "base64_encoded_spanish_audio"
  },
  "options": {
    "language": "es",
    "translate_to_english": true,
    "include_timestamps": true
  }
}
```

**Response:**
```json
{
  "text": "Hello world in English",
  "language": "en",
  "segments": [
    {
      "start": 0.0,
      "end": 2.1,
      "text": "Hello world in English",
      "confidence": null
    }
  ],
  "success": true,
  "error": null,
  "duration_ms": 289,
  "timestamp": "1640995200"
}
```

### Example 5: Beam Search with Custom Parameters

**Request:**
```json
{
  "audio_data": {
    "data": "base64_encoded_audio"
  },
  "options": {
    "language": "en",
    "use_beam_search": true,
    "beam_size": 10,
    "temperature": 0.8,
    "max_tokens": 200
  }
}
```

**Response:**
```json
{
  "text": "Transcribed text with beam search",
  "language": "en",
  "segments": [
    {
      "start": 0.0,
      "end": 3.4,
      "text": "Transcribed text with beam search",
      "confidence": null
    }
  ],
  "success": true,
  "error": null,
  "duration_ms": 445,
  "timestamp": "1640995200"
}
```

## Troubleshooting

### Common JSON Errors

#### 1. Invalid JSON Format

**Error:**
```json
{
  "text": "",
  "language": null,
  "segments": null,
  "success": false,
  "error": "Invalid JSON: expected `}` at line 1 column 10",
  "duration_ms": null,
  "timestamp": "1640995200"
}
```

**Solution:**
- Check JSON syntax for missing brackets, quotes, or commas
- Use a JSON validator to verify your payload
- Ensure all strings are properly quoted

#### 2. Missing Required Fields

**Error:**
```json
{
  "text": "",
  "language": null,
  "segments": null,
  "success": false,
  "error": "Failed to extract audio data: Missing required field: audio_data",
  "duration_ms": null,
  "timestamp": "1640995200"
}
```

**Solution:**
- Ensure your JSON request includes the `audio_data` field
- Verify the field name is spelled correctly
- Check that the audio_data object has the required structure

#### 3. Invalid Base64 Encoding

**Error:**
```json
{
  "text": "",
  "language": null,
  "segments": null,
  "success": false,
  "error": "Failed to decode base64 audio data: Invalid input",
  "duration_ms": null,
  "timestamp": "1640995200"
}
```

**Solution:**
- Verify your base64 string is properly encoded
- Use a base64 encoder/decoder tool to validate your input
- Ensure no extra whitespace or characters are included

#### 4. Invalid Language Code

**Error:**
```json
{
  "text": "",
  "language": null,
  "segments": null,
  "success": false,
  "error": "Invalid value for field 'language': invalid_lang",
  "duration_ms": null,
  "timestamp": "1640995200"
}
```

**Solution:**
- Use a valid language code from the supported list: `["en", "auto", "zh", "de", "es", "ru", "ko", "fr", "ja", "pt", "tr", "pl", "ca"]`
- Check for typos in the language code
- Use `"auto"` for automatic language detection

#### 5. Invalid Temperature Value

**Error:**
```json
{
  "text": "",
  "language": null,
  "segments": null,
  "success": false,
  "error": "Invalid value for field 'temperature': 2.0",
  "duration_ms": null,
  "timestamp": "1640995200"
}
```

**Solution:**
- Ensure temperature is between 0.0 and 1.0
- Use decimal values (e.g., 0.5 instead of 0,5)
- Check for invalid characters in the number

#### 6. Empty Audio Data

**Error:**
```json
{
  "text": "",
  "language": null,
  "segments": null,
  "success": false,
  "error": "Audio data error: Audio data is empty",
  "duration_ms": null,
  "timestamp": "1640995200"
}
```

**Solution:**
- Ensure your audio data is not empty
- Verify the base64 string or binary array contains actual audio data
- Check that audio files are properly formatted (16kHz mono PCM)

### Debugging Tips

1. **Check Logging**: All server logs are sent to stderr. Run your application with stderr capture to see detailed error messages.

2. **Validate Audio Format**: Ensure your audio files are in the correct format (16kHz mono PCM) before processing.

3. **Test with Simple Audio**: Start with simple, short audio clips to verify the interface works before processing complex audio.

4. **Monitor Memory Usage**: Large audio files may consume significant memory. Monitor your system's memory usage during processing.

5. **Check Model Compatibility**: Ensure your Whisper model is compatible with the audio format and language you're trying to transcribe.

### Performance Considerations

- **Base64 vs Binary**: Base64 encoding increases payload size by ~33%. Use binary format for large audio files.
- **Temperature Settings**: Lower temperatures (0.0-0.5) produce more conservative results, while higher temperatures (0.5-1.0) are more creative.
- **Beam Search**: Beam search improves accuracy but increases processing time. Use smaller beam sizes for faster results.
- **Thread Count**: Adjust the `--threads` parameter based on your CPU cores for optimal performance.

## Development

### Building and Testing

```bash
# Check code formatting
cargo fmt --check

# Run linter
cargo clippy

# Run tests
cargo test

# Build debug version
cargo build

# Build release version
cargo build --release
```

### Project Structure

```
whisper-background-server/
├── src/
│   ├── main.rs          # Main application entry point
│   ├── audio.rs         # Audio processing and JSON handling
│   └── transcription.rs # Whisper transcription logic
├── openspec/            # Open specifications and design docs
├── test_output/         # Test output and reports
├── Cargo.toml          # Project dependencies and metadata
└── README.md           # This documentation
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## License

This project is licensed under the Apache-2.0 License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [whisper-rs](https://docs.rs/whisper-rs/) - Whisper.cpp bindings for Rust
- [ggerganov/whisper.cpp](https://github.com/ggerganov/whisper.cpp) - The underlying Whisper implementation
- [Tokio](https://tokio.rs/) - Async runtime for Rust
- [Serde](https://serde.rs/) - Serialization framework for Rust

## Support

For issues, questions, or contributions, please open an issue on the [GitHub repository](https://github.com/your-repo/whisper-background-server).