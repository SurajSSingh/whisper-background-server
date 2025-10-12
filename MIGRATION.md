# Migration Guide: SOT to JSON Interface

## Overview

This guide provides comprehensive instructions for migrating from the SOT (Start of Transmission) marker-based protocol to the new JSON interface in the Whisper Background Server. The migration is designed to be smooth and backward-compatible during the transition period.

## Table of Contents

1. [Migration Overview](#migration-overview)
2. [Key Differences](#key-differences)
3. [Migration Steps](#migration-steps)
4. [Protocol Comparison](#protocol-comparison)
5. [Client Migration Examples](#client-migration-examples)
6. [Common Issues and Solutions](#common-issues-and-solutions)
7. [Deprecation Timeline](#deprecation-timeline)
8. [Support Resources](#support-resources)

## Migration Overview

### Why Migrate to JSON?

The JSON interface offers several advantages over the SOT protocol:

- **Structured Data**: JSON provides a standardized, human-readable format
- **Better Error Handling**: Structured error responses with detailed information
- **Extensibility**: Easy to add new options and features
- **Validation**: Built-in validation for input data
- **Tooling**: Better support from development tools and libraries

### Migration Phases

The migration follows a three-phase approach:

1. **Phase 1: Parallel Implementation** (JSON interface alongside SOT)
2. **Phase 2: Deprecation and Transition** (Encourage JSON adoption)
3. **Phase 3: SOT Protocol Removal** (Next major version)

## Key Differences

### SOT Protocol (Legacy)

```bash
# Binary audio data with SOT marker
echo -ne "audio_data_bytes\0SOT\0" | ./whisper-background-server model.bin
```

**Characteristics:**
- Binary format with null-terminated SOT marker
- Simple but limited structure
- No configuration options
- Error handling limited to basic responses
- Audio data must be sent as complete chunks

### JSON Interface (New)

```json
{
  "audio_data": {
    "data": "base64_encoded_audio_string"
  },
  "options": {
    "language": "en",
    "include_timestamps": true,
    "temperature": 0.0
  }
}
```

**Characteristics:**
- Structured JSON format
- Comprehensive configuration options
- Detailed error responses
- Support for both base64 and binary audio data
- Better validation and type safety

## Migration Steps

### Step 1: Assessment

1. **Inventory Current Clients**
   - Identify all applications using the SOT protocol
   - Document integration points and usage patterns
   - Assess migration complexity for each client

2. **Create Migration Plan**
   - Prioritize clients based on usage and criticality
   - Set migration timeline for each client
   - Assign responsibilities and resources

### Step 2: Testing with JSON Interface

1. **Test Environment Setup**
   ```bash
   # Build the server with JSON support
   cargo build --release
   ```

2. **Basic JSON Testing**
   ```bash
   # Create a test JSON request
   cat > test_request.json << EOF
   {
     "audio_data": {
       "data": "$(base64 -i test_audio.wav)"
     }
   }
   EOF
   
   # Test the JSON interface
   cat test_request.json | ./target/release/whisper-background-server model.bin
   ```

3. **Validate Functionality**
   - Test with different audio formats
   - Verify transcription accuracy
   - Test error scenarios and edge cases

### Step 3: Client Migration

1. **Update Client Code**
   - Replace SOT detection with JSON parsing
   - Update request format to JSON structure
   - Implement proper error handling for JSON responses

2. **Configuration Updates**
   - Add support for transcription options
   - Implement validation for JSON payloads
   - Add logging for debugging

3. **Testing and Validation**
   - Unit tests for JSON parsing
   - Integration tests with the server
   - Performance testing to ensure no regression

### Step 4: Deployment

1. **Staging Deployment**
   - Deploy updated clients to staging environment
   - Monitor for issues and performance impacts
   - Gather feedback from users

2. **Production Deployment**
   - Deploy with rollback plan
   - Monitor system performance
   - Provide support for migration issues

## Protocol Comparison

### Request Format

| Aspect | SOT Protocol | JSON Interface |
|--------|-------------|----------------|
| Format | Binary with SOT marker | Structured JSON |
| Audio Data | Raw binary bytes | Base64 string or binary array |
| Configuration | None | Comprehensive options |
| Validation | Basic | Comprehensive field validation |
| Error Handling | Basic responses | Structured error details |

### Response Format

| Aspect | SOT Protocol | JSON Interface |
|--------|-------------|----------------|
| Format | Binary/text | Structured JSON |
| Information | Basic transcription | Detailed results with metadata |
| Error Handling | Simple error messages | Structured error responses |
| Metadata | Limited | Comprehensive (timestamps, duration, etc.) |

### Audio Data Support

| Feature | SOT Protocol | JSON Interface |
|---------|-------------|----------------|
| Audio Format | 16kHz mono PCM only | 16kHz mono PCM |
| Encoding | Raw binary | Base64 or binary |
| Chunking | Required with SOT markers | Complete JSON payload |
| Validation | Basic format check | Comprehensive validation |

## Client Migration Examples

### Example 1: Basic Client Migration

#### Before (SOT Protocol)

```rust
// Legacy SOT-based client
use std::io::{self, Read, Write};
use std::process::Command;

fn transcribe_with_sot(audio_data: &[u8]) -> Result<String, String> {
    let mut child = Command::new("./whisper-background-server")
        .arg("model.bin")
        .stdin(io::piped())
        .stdout(io::piped())
        .stderr(io::piped())
        .spawn()
        .map_err(|e| format!("Failed to start server: {}", e))?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(audio_data).map_err(|e| format!("Failed to write audio: {}", e))?;
        stdin.write_all(b"\0SOT\0").map_err(|e| format!("Failed to write SOT: {}", e))?;
    }

    let output = child.wait_with_output().map_err(|e| format!("Failed to get output: {}", e))?;
    
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
```

#### After (JSON Interface)

```rust
// New JSON-based client
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::process::Command;

#[derive(Serialize)]
struct TranscriptionRequest {
    audio_data: AudioData,
    options: Option<TranscriptionOptions>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum AudioData {
    Base64 { data: String, format: Option<String> },
    Binary { data: Vec<u8>, format: Option<String> },
}

#[derive(Serialize, Default)]
struct TranscriptionOptions {
    language: Option<String>,
    include_timestamps: Option<bool>,
    temperature: Option<f32>,
}

#[derive(Deserialize)]
struct TranscriptionResult {
    text: String,
    language: Option<String>,
    segments: Option<Vec<Segment>>,
    success: bool,
    error: Option<String>,
    duration_ms: Option<u64>,
    timestamp: Option<String>,
}

#[derive(Deserialize)]
struct Segment {
    start: f32,
    end: f32,
    text: String,
    confidence: Option<f32>,
}

fn transcribe_with_json(audio_data: &[u8]) -> Result<String, String> {
    let request = TranscriptionRequest {
        audio_data: AudioData::Base64 {
            data: base64::Engine::encode(&base64::engine::GeneralPurpose::new(&base64::alphabet::STANDARD), audio_data),
            format: Some("pcm".to_string()),
        },
        options: Some(TranscriptionOptions {
            language: Some("en".to_string()),
            include_timestamps: Some(true),
            temperature: Some(0.0),
            ..Default::default()
        }),
    };

    let json_request = serde_json::to_string(&request)
        .map_err(|e| format!("Failed to serialize request: {}", e))?;

    let mut child = Command::new("./whisper-background-server")
        .arg("model.bin")
        .stdin(io::piped())
        .stdout(io::piped())
        .stderr(io::piped())
        .spawn()
        .map_err(|e| format!("Failed to start server: {}", e))?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(json_request.as_bytes())
            .map_err(|e| format!("Failed to write request: {}", e))?;
    }

    let output = child.wait_with_output().map_err(|e| format!("Failed to get output: {}", e))?;
    
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let result: TranscriptionResult = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if result.success {
        Ok(result.text)
    } else {
        Err(result.error.unwrap_or_else(|| "Unknown error".to_string()))
    }
}
```

### Example 2: Shell Script Migration

#### Before (SOT Protocol)

```bash
#!/bin/bash
# Legacy SOT-based transcription script

if [ $# -lt 2 ]; then
    echo "Usage: $0 <model_path> <audio_file>"
    exit 1
fi

MODEL_PATH="$1"
AUDIO_FILE="$2"

# Check if audio file exists
if [ ! -f "$AUDIO_FILE" ]; then
    echo "Error: Audio file not found: $AUDIO_FILE"
    exit 1
fi

# Send audio data with SOT marker
cat "$AUDIO_FILE" | stdbuf -i0 -o0 -e0 \
    python3 -c "
import sys
import os
sys.stdin.buffer.write(os.urandom(1024))  # Dummy audio data
sys.stdin.buffer.write(b'\0SOT\0')
" | ./whisper-background-server "$MODEL_PATH"
```

#### After (JSON Interface)

```bash
#!/bin/bash
# New JSON-based transcription script

if [ $# -lt 2 ]; then
    echo "Usage: $0 <model_path> <audio_file> [language]"
    exit 1
fi

MODEL_PATH="$1"
AUDIO_FILE="$2"
LANGUAGE="${3:-auto}"

# Check if audio file exists
if [ ! -f "$AUDIO_FILE" ]; then
    echo "Error: Audio file not found: $AUDIO_FILE"
    exit 1
fi

# Create JSON request
JSON_REQUEST=$(cat <<EOF
{
  "audio_data": {
    "data": "$(base64 -i "$AUDIO_FILE")",
    "format": "wav"
  },
  "options": {
    "language": "$LANGUAGE",
    "include_timestamps": true,
    "temperature": 0.0
  }
}
EOF
)

# Send JSON request
echo "$JSON_REQUEST" | ./whisper-background-server "$MODEL_PATH"
```

### Example 3: Python Client Migration

#### Before (SOT Protocol)

```python
# Legacy SOT-based Python client
import subprocess
import sys

def transcribe_sot(audio_data, model_path):
    """
    Transcribe audio using SOT protocol
    
    Args:
        audio_data: Raw audio bytes
        model_path: Path to Whisper model
        
    Returns:
        str: Transcribed text
    """
    try:
        # Start the server process
        process = subprocess.Popen(
            ['./whisper-background-server', model_path],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE
        )
        
        # Send audio data with SOT marker
        process.stdin.write(audio_data)
        process.stdin.write(b'\0SOT\0')
        process.stdin.close()
        
        # Get output
        stdout, stderr = process.communicate()
        
        if process.returncode != 0:
            raise Exception(f"Server error: {stderr.decode()}")
            
        return stdout.decode().strip()
        
    except Exception as e:
        raise Exception(f"Transcription failed: {str(e)}")
```

#### After (JSON Interface)

```python
# New JSON-based Python client
import json
import base64
import subprocess
import sys
from typing import Optional, Dict, Any

class TranscriptionOptions:
    def __init__(
        self,
        language: Optional[str] = None,
        include_timestamps: bool = True,
        temperature: float = 0.0,
        translate_to_english: bool = False,
        max_tokens: Optional[int] = None,
        use_beam_search: bool = False,
        beam_size: Optional[int] = None,
        suppress_blank: bool = True,
        word_timestamps: bool = False
    ):
        self.language = language
        self.include_timestamps = include_timestamps
        self.temperature = temperature
        self.translate_to_english = translate_to_english
        self.max_tokens = max_tokens
        self.use_beam_search = use_beam_search
        self.beam_size = beam_size
        self.suppress_blank = suppress_blank
        self.word_timestamps = word_timestamps

    def to_dict(self) -> Dict[str, Any]:
        options = {}
        if self.language is not None:
            options['language'] = self.language
        if self.include_timestamps is not None:
            options['include_timestamps'] = self.include_timestamps
        if self.temperature is not None:
            options['temperature'] = self.temperature
        if self.translate_to_english is not None:
            options['translate_to_english'] = self.translate_to_english
        if self.max_tokens is not None:
            options['max_tokens'] = self.max_tokens
        if self.use_beam_search is not None:
            options['use_beam_search'] = self.use_beam_search
        if self.beam_size is not None:
            options['beam_size'] = self.beam_size
        if self.suppress_blank is not None:
            options['suppress_blank'] = self.suppress_blank
        if self.word_timestamps is not None:
            options['word_timestamps'] = self.word_timestamps
        return options

class TranscriptionResult:
    def __init__(self, data: Dict[str, Any]):
        self.text = data.get('text', '')
        self.language = data.get('language')
        self.segments = data.get('segments')
        self.success = data.get('success', False)
        self.error = data.get('error')
        self.duration_ms = data.get('duration_ms')
        self.timestamp = data.get('timestamp')

def transcribe_json(
    audio_data: bytes,
    model_path: str,
    options: Optional[TranscriptionOptions] = None
) -> TranscriptionResult:
    """
    Transcribe audio using JSON interface
    
    Args:
        audio_data: Raw audio bytes
        model_path: Path to Whisper model
        options: Transcription options
        
    Returns:
        TranscriptionResult: Transcription result with metadata
    """
    try:
        # Prepare request
        request = {
            'audio_data': {
                'data': base64.b64encode(audio_data).decode('utf-8'),
                'format': 'wav'
            }
        }
        
        if options:
            request['options'] = options.to_dict()
        
        json_request = json.dumps(request)
        
        # Start the server process
        process = subprocess.Popen(
            ['./whisper-background-server', model_path],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        
        # Send JSON request
        stdout, stderr = process.communicate(input=json_request)
        
        if process.returncode != 0:
            raise Exception(f"Server error: {stderr}")
        
        # Parse JSON response
        response_data = json.loads(stdout)
        return TranscriptionResult(response_data)
        
    except json.JSONDecodeError as e:
        raise Exception(f"Invalid JSON response: {str(e)}")
    except Exception as e:
        raise Exception(f"Transcription failed: {str(e)}")

# Example usage
if __name__ == "__main__":
    # Example with audio file
    with open('test_audio.wav', 'rb') as f:
        audio_data = f.read()
    
    options = TranscriptionOptions(
        language='en',
        include_timestamps=True,
        temperature=0.0
    )
    
    result = transcribe_json(audio_data, 'model.bin', options)
    
    if result.success:
        print(f"Transcription: {result.text}")
        if result.language:
            print(f"Language: {result.language}")
        if result.duration_ms:
            print(f"Duration: {result.duration_ms}ms")
    else:
        print(f"Error: {result.error}")
```

## Common Issues and Solutions

### Issue 1: JSON Parsing Errors

**Problem:**
```
Invalid JSON: expected `}` at line 1 column 10
```

**Solution:**
- Validate JSON syntax using a JSON validator
- Ensure all strings are properly quoted
- Check for missing commas or brackets
- Use JSON linting tools in your development environment

**Prevention:**
```python
# Python example with validation
import json

def validate_json(json_str):
    try:
        json.loads(json_str)
        return True, None
    except json.JSONDecodeError as e:
        return False, str(e)

# Usage
is_valid, error = validate_json(json_request)
if not is_valid:
    print(f"Invalid JSON: {error}")
```

### Issue 2: Base64 Encoding Problems

**Problem:**
```
Failed to decode base64 audio data: Invalid input
```

**Solution:**
- Ensure base64 strings don't contain extra whitespace
- Use proper base64 encoding for your audio data
- Validate base64 strings before sending

**Prevention:**
```bash
# Clean base64 encoding
base64 -i audio.wav | tr -d '\n\r' > audio_base64.txt
```

### Issue 3: Audio Format Issues

**Problem:**
```
Audio data error: Audio data is empty
```

**Solution:**
- Verify audio files are in correct format (16kHz mono PCM)
- Check that audio files are not corrupted
- Ensure proper file permissions

**Prevention:**
```python
# Python audio validation
import wave
import struct

def validate_audio_format(audio_data):
    try:
        # Check if audio data is valid WAV format
        if len(audio_data) < 44:  # Minimum WAV header size
            return False, "Audio data too short"
        
        # Check WAV header
        if audio_data[:4] != b'RIFF':
            return False, "Invalid WAV format"
            
        return True, "Valid audio format"
    except Exception as e:
        return False, str(e)
```

### Issue 4: Language Code Validation

**Problem:**
```
Invalid value for field 'language': invalid_lang
```

**Solution:**
- Use valid language codes from the supported list
- Check for typos in language codes
- Use "auto" for automatic language detection

**Valid Language Codes:**
```json
["en", "auto", "zh", "de", "es", "ru", "ko", "fr", "ja", "pt", "tr", "pl", "ca"]
```

### Issue 5: Temperature Range Errors

**Problem:**
```
Invalid value for field 'temperature': 2.0
```

**Solution:**
- Ensure temperature is between 0.0 and 1.0
- Use decimal values (e.g., 0.5 instead of 0,5)
- Check for invalid characters in numeric values

### Issue 6: Memory Issues with Large Audio

**Problem:**
- High memory usage
- Slow processing
- Potential crashes

**Solution:**
- Use binary format instead of base64 for large files
- Process audio in chunks if possible
- Monitor memory usage during processing

**Optimization:**
```python
# For large audio files, use binary format
request = {
    'audio_data': {
        'data': list(audio_bytes),  # Convert to list for JSON serialization
        'format': 'wav'
    }
}
```

### Issue 7: Error Handling in Clients

**Problem:**
- Poor error handling leads to unclear failure messages
- Clients don't handle server errors gracefully

**Solution:**
- Implement comprehensive error handling
- Parse and handle structured error responses
- Provide meaningful error messages to users

**Example:**
```python
def handle_transcription_error(result):
    if not result.success:
        if result.error:
            if "Invalid JSON" in result.error:
                return "Invalid request format. Please check your JSON syntax."
            elif "audio_data" in result.error:
                return "Audio data is missing or invalid."
            elif "language" in result.error:
                return "Invalid language code specified."
            else:
                return f"Transcription error: {result.error}"
        else:
            return "Unknown transcription error."
    return None
```

## Deprecation Timeline

### Phase 1: Parallel Implementation (Complete)
- ✅ JSON interface implemented alongside SOT protocol
- ✅ Both protocols functional
- ✅ Basic testing completed

### Phase 2: Deprecation and Transition (Current)
- ⚠️ **SOT protocol deprecated with warnings**
- ⚠️ **Migration tools available**
- ⚠️ **Client migration encouraged**

**Current Status:**
- SOT protocol still works but shows deprecation warnings
- JSON interface is fully functional and recommended
- Migration scripts and examples available

### Phase 3: SOT Protocol Removal (Future)
- 📅 **Target: Next major version (v2.0.0)**
- 📅 **Estimated: Q1 2025**
- 📅 **Final migration deadline: TBA**

**What to Expect:**
- SOT protocol completely removed
- JSON interface becomes the only supported protocol
- Breaking changes in version number
- Updated documentation and examples

## Support Resources

### Documentation
- [JSON Interface Specification](openspec/changes/refactor-json-interface/specs/audio-input/spec.md)
- [Migration Plan](openspec/changes/refactor-json-interface/migration-plan.md)
- [API Reference](README.md#json-interface)

### Tools and Scripts
- [SOT to JSON Converter](sot_to_json_converter.rs)
- [Python Migration Example](migration_example.py)
- [Test Scripts](test_json_interface.sh)

### Community Support
- GitHub Issues: Report bugs and request features
- Discussions: Share migration experiences and ask questions
- Wiki: Community-contributed tips and examples

### Migration Checklist

- [ ] Inventory all SOT-based clients
- [ ] Test JSON interface with your use case
- [ ] Update client code to use JSON interface
- [ ] Implement proper error handling
- [ ] Test with real audio files
- [ ] Deploy to staging environment
- [ ] Monitor for issues in production
- [ ] Provide feedback and report issues

### Best Practices

1. **Start Early**: Begin migration as soon as possible
2. **Test Thoroughly**: Test with real audio data and edge cases
3. **Monitor Performance**: Watch for performance regressions
4. **Handle Errors Gracefully**: Implement comprehensive error handling
5. **Document Changes**: Keep documentation updated with changes
6. **Communicate Changes**: Inform users about the migration
7. **Provide Support**: Be ready to help with migration issues

### Contact Information

For migration support:
- GitHub Issues: [Project Repository Issues](https://github.com/your-repo/whisper-background-server/issues)
- Documentation: [Project Wiki](https://github.com/your-repo/whisper-background-server/wiki)
- Community: [GitHub Discussions](https://github.com/your-repo/whisper-background-server/discussions)

---

*This migration guide will be updated as the migration progresses. Please check for the latest version before starting your migration.*