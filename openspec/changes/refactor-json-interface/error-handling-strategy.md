# Error Handling and Validation Strategy

## Overview
This document outlines the comprehensive error handling and validation strategy for the new JSON interface in the Whisper Background Server. The strategy ensures robust input validation, clear error reporting, and graceful error recovery while maintaining system stability.

## Error Handling Principles

### 1. Fail Fast, Fail Clear
- Validate JSON structure immediately upon receipt
- Provide specific error messages for validation failures
- Never attempt processing with invalid input

### 2. Structured Error Responses
- All errors returned in consistent JSON format on stdout
- Error responses include error codes, messages, and optional context
- Maintain compatibility with existing error output structure

### 3. Logging Separation
- All error details logged to stderr only
- stdout reserved for JSON responses (success or error)
- No logging interference with JSON parsing by calling applications

### 4. Graceful Degradation
- System remains operational after processing errors
- Invalid JSON payloads don't crash the application
- Resource cleanup for failed operations

## Error Response Format

### Standard Error Response
```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Missing required field: audio_data",
    "details": {
      "field": "audio_data",
      "expected": "base64 string or binary array",
      "received": null
    }
  },
  "timestamp": "2024-01-01T12:00:00Z"
}
```

### Transcription Error Response
```json
{
  "success": false,
  "error": {
    "code": "TRANSCRIPTION_ERROR",
    "message": "Whisper processing failed: Invalid audio format",
    "details": {
      "audio_size_bytes": 0,
      "whisper_error": "Audio data is empty"
    }
  },
  "timestamp": "2024-01-01T12:00:00Z"
}
```

## Error Categories and Codes

### 1. JSON Parsing Errors (1000-1999)
- `1001`: JSON syntax error
- `1002`: Incomplete JSON payload
- `1003`: JSON depth exceeded

### 2. Validation Errors (2000-2999)
- `2001`: Missing required field
- `2002`: Invalid field type
- `2003`: Invalid audio data format
- `2004`: Invalid transcription option
- `2005`: Invalid language code
- `2006`: Parameter value out of range

### 3. Audio Processing Errors (3000-3999)
- `3001`: Empty audio data
- `3002`: Audio data too large
- `3003`: Invalid audio format
- `3004`: Audio data corruption

### 4. Transcription Errors (4000-4999)
- `4001`: Whisper context error
- `4002`: Transcription processing failed
- `4003`: Model loading error
- `4004`: Configuration error

## Validation Strategy

### 1. JSON Structure Validation
```rust
// Validate top-level structure
{
  "audio_data": required, // string or array
  "options": optional,   // object
}

// Validate options structure
{
  "language": optional,           // string
  "translate_to_english": optional, // boolean
  "include_timestamps": optional,   // boolean
  "max_tokens": optional,          // number
  "temperature": optional,         // number (0.0-1.0)
  "use_beam_search": optional,     // boolean
  "beam_size": optional,           // number (1-10)
  "suppress_blank": optional,      // boolean
  "word_timestamps": optional,     // boolean
}
```

### 2. Audio Data Validation
- **Base64 format**: Validate string encoding and decode to bytes
- **Binary format**: Validate array of u8 values
- **Size limits**: Check against configurable maximum (default: 100MB)
- **Format validation**: Ensure 16kHz mono PCM compatibility

### 3. Transcription Option Validation
- **Language codes**: Validate against supported languages
- **Numeric ranges**: Check temperature (0.0-1.0), beam_size (1-10), etc.
- **Type validation**: Ensure boolean flags are actually boolean
- **Conflicting options**: Detect and report invalid combinations

## Implementation Strategy

### 1. Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum JsonInputError {
    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),
    
    #[error("Validation error: {code} - {message}")]
    Validation { code: u16, message: String, details: Option<serde_json::Value> },
    
    #[error("Audio data error: {0}")]
    AudioData(String),
    
    #[error("Transcription error: {0}")]
    Transcription(#[from] transcription::TranscriptionError),
}
```

### 2. Validation Functions
```rust
// Validate complete JSON payload
fn validate_json_payload(payload: &serde_json::Value) -> Result<(), JsonInputError> {
    validate_required_fields(payload)?;
    validate_audio_data(payload)?;
    validate_transcription_options(payload)?;
    Ok(())
}

// Validate audio data format
fn validate_audio_data(payload: &serde_json::Value) -> Result<(), JsonInputError> {
    let audio_data = payload.get("audio_data")
        .ok_or_else(|| JsonInputError::validation(2001, "Missing required field: audio_data", None))?;
    
    match audio_data {
        serde_json::Value::String(base64_str) => validate_base64_audio(base64_str)?,
        serde_json::Value::Array(bytes) => validate_binary_audio(bytes)?,
        _ => return Err(JsonInputError::validation(2003, "Invalid audio data format", None)),
    }
    
    Ok(())
}
```

### 3. Error Response Generation
```rust
// Generate structured error response
fn generate_error_response(error: JsonInputError) -> TranscriptionOutput {
    let (code, message, details) = match error {
        JsonInputError::JsonParse(e) => (1001, format!("JSON syntax error: {}", e), None),
        JsonInputError::Validation { code, message, details } => (code, message, details),
        JsonInputError::AudioData(msg) => (3001, format!("Audio data error: {}", msg), None),
        JsonInputError::Transcription(e) => (4002, format!("Transcription error: {}", e), None),
    };
    
    TranscriptionOutput {
        text: String::new(),
        language: None,
        segments: None,
        success: false,
        error: Some(message),
        duration_ms: None,
        timestamp: Some(chrono::Utc::now().to_rfc3339()),
    }
}
```

## Logging Strategy

### 1. Error Logging Levels
- **ERROR**: Critical failures that prevent processing
- **WARN**: Validation failures and recoverable errors
- **INFO**: Successful processing and important events
- **DEBUG**: Detailed validation and processing information

### 2. Log Message Format
```
[timestamp] LEVEL [module] Error details with context
```

### 3. Sensitive Data Handling
- Never log actual audio data in error messages
- Sanitize error messages to prevent information leakage
- Log error codes and contextual information only

## Recovery and Resilience

### 1. Input Recovery
- Handle incomplete JSON payloads gracefully
- Reset state after processing errors
- Continue accepting new requests after failures

### 2. Resource Management
- Proper cleanup of failed operations
- Memory management for large audio files
- Timeout handling for slow operations

### 3. Monitoring and Metrics
- Track error rates by error code
- Monitor validation failure patterns
- Alert on high error rates or unusual error patterns

## Testing Strategy

### 1. Unit Testing
- Test individual validation functions
- Verify error code generation
- Test error message formatting

### 2. Integration Testing
- Test complete error handling workflows
- Verify error response format compliance
- Test logging separation

### 3. Error Scenario Testing
- Test malformed JSON inputs
- Test invalid audio data formats
- Test edge cases and boundary conditions