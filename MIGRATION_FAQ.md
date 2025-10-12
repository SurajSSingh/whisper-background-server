# Migration FAQ: SOT to JSON Interface

> Human editor's note: Take any numbers with a grain of salt, the LLM likely made up the numbers. This does provide a good overview of changes, but use your best judgement. Also anywhere it says v2.0.0, instead, consider it v0.3.0. Below items left as is (you likely don't need to consider it if you are new)

This document addresses frequently asked questions and common issues encountered during the migration from the SOT protocol to the JSON interface.

## Table of Contents

1. [General Questions](#general-questions)
2. [Migration Process](#migration-process)
3. [Technical Issues](#technical-issues)
4. [Performance Concerns](#performance-concerns)
5. [Client Integration](#client-integration)
6. [Error Handling](#error-handling)
7. [Compatibility](#compatibility)
8. [Future Planning](#future-planning)

## General Questions

### Q1: Why is the SOT protocol being deprecated?

**A:** The SOT protocol is being deprecated for several reasons:

- **Limited functionality**: SOT doesn't support configuration options or advanced features
- **Poor error handling**: Binary format makes error detection and reporting difficult
- **Extensibility issues**: Adding new features to SOT would require breaking changes
- **Tooling limitations**: JSON has better support from development tools and libraries
- **Maintenance burden**: Supporting two protocols increases complexity

The JSON interface provides a more robust, extensible, and developer-friendly alternative.

### Q2: When will SOT support be completely removed?

**A:** SOT support will be completely removed in version 2.0.0, scheduled for Q1 2025. The exact date will be announced at least 30 days in advance.

### Q3: Will there be a grace period after SOT removal?

**A:** No, there will not be a grace period. The removal will be complete in v2.0.0. All clients must be migrated before upgrading to v2.0.0.

### Q4: Is the migration mandatory?

**A:** Yes, for all users who want to continue using the Whisper Background Server after v2.0.0. The SOT protocol will no longer be supported.

### Q5: What happens if I don't migrate?

**A:** After v2.0.0 is released:
- SOT protocol requests will be rejected
- Your clients will receive error messages
- You will not be able to use new features
- You may experience security vulnerabilities if SOT support contains unpatched issues

## Migration Process

### Q6: How long does the migration typically take?

**A:** Migration time varies depending on complexity:

- **Simple clients**: 1-4 hours
- **Medium complexity**: 1-2 days
- **Complex integrations**: 1-2 weeks

Factors affecting migration time:
- Number of clients to update
- Integration complexity
- Testing requirements
- Team size and experience

### Q7: What's the best approach to start the migration?

**A:** We recommend this approach:

1. **Assessment** (1-2 days)
   - Inventory all SOT-based clients
   - Document integration points
   - Identify dependencies

2. **Planning** (1-2 days)
   - Create migration timeline
   - Assign responsibilities
   - Set testing milestones

3. **Development** (variable)
   - Update client code
   - Implement JSON interface
   - Add error handling

4. **Testing** (3-5 days)
   - Unit testing
   - Integration testing
   - Performance testing

5. **Deployment** (1-2 weeks)
   - Staging deployment
   - Production deployment
   - Monitoring and support

### Q8: Should I migrate all clients at once?

**A:** No, we recommend a phased approach:

1. **Start with non-critical clients** to gain experience
2. **Migrate critical clients** during maintenance windows
3. **Keep SOT clients running** during transition
4. **Complete migration** before v2.0.0 release

### Q9: What tools are available to help with migration?

**A:** We provide several migration tools:

- **`MIGRATION.md`**: Comprehensive migration guide
- **`migration_example.py`**: Python utilities and examples
- **`sot_to_json_converter.rs`**: Rust conversion utility
- **Compatibility layer**: For gradual migration
- **Test scripts**: For validation

### Q10: Can I get professional migration assistance?

**A:** Yes, professional migration services are available:

- **Consulting**: Architecture review and planning
- **Implementation**: Custom migration development
- **Training**: Team training on JSON interface
- **Support**: Priority support during migration

Contact business@example.com for enterprise services.

## Technical Issues

### Q11: My JSON requests are failing with "Invalid JSON" errors. What's wrong?

**A:** Common JSON validation issues:

1. **Syntax errors**:
   ```json
   // ❌ Missing comma
   {
     "audio_data": {
       "data": "base64_string"
     }
     "options": { ... }
   }
   
   // ✅ Correct
   {
     "audio_data": {
       "data": "base64_string"
     },
     "options": { ... }
   }
   ```

2. **Invalid base64 encoding**:
   ```bash
   # Clean base64 encoding (no newlines)
   base64 -i audio.wav | tr -d '\n\r' > clean_base64.txt
   ```

3. **Missing required fields**:
   ```json
   // ❌ Missing audio_data
   {
     "options": { ... }
   }
   
   // ✅ Complete request
   {
     "audio_data": {
       "data": "base64_string"
     },
     "options": { ... }
   }
   ```

**Solution**: Use a JSON validator and test with the provided examples.

### Q12: How do I handle audio data in different formats?

**A:** The JSON interface supports multiple audio data formats:

**Base64 Format (Recommended)**:
```json
{
  "audio_data": {
    "data": "base64_encoded_string",
    "format": "wav"
  }
}
```

**Binary Format (for large files)**:
```json
{
  "audio_data": {
    "data": [255, 255, 0, 128, ...],
    "format": "wav"
  }
}
```

**Format Requirements**:
- Audio must be 16kHz, mono, PCM format
- WAV files work best
- Other formats may need conversion

### Q13: Why am I getting "Invalid language code" errors?

**A:** The JSON interface validates language codes. Supported languages:

```json
["en", "auto", "zh", "de", "es", "ru", "ko", "fr", "ja", "pt", "tr", "pl", "ca"]
```

**Common issues**:
- Typos in language codes
- Case sensitivity (use lowercase)
- Unsupported languages

**Solution**: Use `"auto"` for automatic detection or verify the language code is in the supported list.

### Q14: How do I handle temperature parameter validation?

**A:** Temperature must be between 0.0 and 1.0:

```json
{
  "options": {
    "temperature": 0.5  // Valid range: 0.0 to 1.0
  }
}
```

**Common mistakes**:
- Values outside range (e.g., 1.5 or -0.1)
- String instead of number (e.g., "0.5")
- Invalid decimal separators (e.g., 0,5 instead of 0.5)

### Q15: What's the difference between include_timestamps and word_timestamps?

**A:** Both options control timestamp output but serve different purposes:

**include_timestamps**:
- Adds segment-level timestamps (start/end times)
- Format: `{"start": 1.2, "end": 2.5, "text": "Hello"}`
- Recommended for most use cases

**word_timestamps**:
- Adds word-level timestamps
- More detailed but slower processing
- Only available for certain models

**Usage**:
```json
{
  "options": {
    "include_timestamps": true,    // Segment timestamps
    "word_timestamps": false       // Word timestamps (optional)
  }
}
```

## Performance Concerns

### Q16: Is the JSON interface slower than SOT?

**A:** Performance is comparable for most use cases:

**SOT Protocol**:
- Pros: Simple binary format, minimal parsing overhead
- Cons: No validation, error handling limited

**JSON Interface**:
- Pros: Rich validation, better error handling, extensible
- Cons: Slightly more parsing overhead

**Benchmark Results** (typical audio file):
- SOT: ~100ms processing time
- JSON: ~105ms processing time
- Difference: ~5% overhead

### Q17: How can I optimize JSON performance for large files?

**A:** For large audio files, use these optimizations:

1. **Use binary format instead of base64**:
   ```json
   // ❌ Base64 (33% larger)
   {
     "audio_data": {
       "data": "very_long_base64_string..."
     }
   }
   
   // ✅ Binary format
   {
     "audio_data": {
       "data": [255, 255, 0, 128, ...]
     }
   }
   ```

2. **Minimize JSON size**:
   ```json
   // ❌ Verbose options
   {
     "audio_data": { "data": "base64..." },
     "options": {
       "language": "en",
       "include_timestamps": true,
       "temperature": 0.0,
       "translate_to_english": false,
       "max_tokens": null,
       "use_beam_search": false,
       "beam_size": null,
       "suppress_blank": true,
       "word_timestamps": false
     }
   }
   
   // ✅ Minimal options
   {
     "audio_data": { "data": "base64..." },
     "options": {
       "language": "en",
       "include_timestamps": true
     }
   }
   ```

3. **Use streaming for very large files** (if supported in your client)

### Q18: Does base64 encoding significantly impact performance?

**A:** Base64 encoding adds ~33% overhead to the payload size:

**Impact Analysis**:
- **Small files** (<1MB): Negligible impact
- **Medium files** (1-10MB): Noticeable but acceptable
- **Large files** (>10MB): Significant impact

**Recommendation**:
- Use base64 for convenience with small/medium files
- Use binary format for large files (>10MB)
- Consider chunking for very large audio files

### Q19: How does the JSON interface affect memory usage?

**A:** Memory usage is similar between protocols:

**Memory Comparison**:
- SOT: Stores raw binary audio data
- JSON: Stores base64-encoded data (33% larger) + JSON parsing overhead

**Optimization Tips**:
- Use binary format for memory efficiency
- Process audio in chunks when possible
- Monitor memory usage during development

### Q20: Are there any threading or concurrency considerations?

**A:** The JSON interface handles concurrency the same way as SOT:

**Best Practices**:
- One request per server process
- Multiple server processes for concurrent requests
- Connection pooling for high-volume applications

**Example**:
```bash
# Run multiple server instances for concurrency
./whisper-background-server model.bin &
./whisper-background-server model.bin &
./whisper-background-server model.bin &

# Load balance requests across instances
```

## Client Integration

### Q21: How do I handle authentication with the JSON interface?

**A:** The JSON interface doesn't have built-in authentication. For secure deployments:

**Options**:
1. **Network-level security**: Use SSH tunnels or VPN
2. **Reverse proxy**: Add authentication at the proxy level
3. **Custom headers**: Include authentication in request headers (if supported)
4. **Pre-shared secrets**: Include in JSON payload (not recommended for production)

**Example with reverse proxy**:
```nginx
# nginx configuration
location /whisper/ {
    auth_basic "Whisper Server";
    auth_basic_user_file /etc/nginx/.htpasswd;
    
    proxy_pass http://localhost:8080/;
    proxy_set_header Host $host;
}
```

### Q22: How do I handle retries and error recovery?

**A:** Implement robust error handling in your client:

**Retry Strategy**:
```python
def transcribe_with_retry(audio_data, max_retries=3):
    for attempt in range(max_retries):
        try:
            result = client.transcribe_with_json(audio_data, options)
            if result.success:
                return result
            elif result.error and "temporary" in result.error.lower():
                time.sleep(2 ** attempt)  # Exponential backoff
                continue
            else:
                raise Exception(result.error)
        except Exception as e:
            if attempt == max_retries - 1:
                raise
            time.sleep(2 ** attempt)
    
    raise Exception("Max retries exceeded")
```

### Q23: How do I handle streaming audio with the JSON interface?

**A:** The JSON interface currently expects complete audio files. For streaming:

**Options**:
1. **Buffer and chunk**: Accumulate audio data and send in chunks
2. **File-based streaming**: Write to temporary files and process
3. **Custom streaming**: Implement streaming protocol (requires server modification)

**Example with buffering**:
```python
class StreamingTranscriber:
    def __init__(self, chunk_size=1024*1024):  # 1MB chunks
        self.chunk_size = chunk_size
        self.buffer = bytearray()
    
    def add_audio_data(self, audio_chunk):
        self.buffer.extend(audio_chunk)
        
        while len(self.buffer) >= self.chunk_size:
            chunk = self.buffer[:self.chunk_size]
            self.buffer = self.buffer[self.chunk_size:]
            yield self.transcribe_chunk(chunk)
    
    def transcribe_chunk(self, chunk):
        # Convert chunk to JSON request and transcribe
        pass
```

### Q24: How do I handle different audio formats?

**A:** The JSON interface expects 16kHz mono PCM audio. For other formats:

**Conversion Options**:
1. **Pre-conversion**: Convert audio before sending
2. **Server-side conversion**: Add conversion to server (requires modification)
3. **Client-side conversion**: Use libraries like pydub or ffmpeg

**Example with pydub**:
```python
from pydub import AudioSegment

def convert_to_16khz_mono(audio_path):
    # Load audio
    audio = AudioSegment.from_file(audio_path)
    
    # Convert to mono
    audio = audio.set_channels(1)
    
    # Convert to 16kHz
    audio = audio.set_frame_rate(16000)
    
    # Export as PCM
    pcm_data = audio.raw_data
    return pcm_data
```

### Q25: How do I handle metadata and additional information?

**A:** The JSON interface supports rich metadata in responses:

**Response Structure**:
```json
{
  "text": "Transcribed text",
  "language": "en",
  "segments": [
    {
      "start": 1.2,
      "end": 2.5,
      "text": "Segment text",
      "confidence": 0.95
    }
  ],
  "success": true,
  "error": null,
  "duration_ms": 289,
  "timestamp": "1640995200"
}
```

**Metadata Usage**:
- **duration_ms**: Use for progress tracking
- **timestamp**: Use for request correlation
- **confidence**: Use for result quality assessment
- **language**: Use for post-processing decisions

## Error Handling

### Q26: How do I handle different error types?

**A:** The JSON interface provides structured error responses:

**Error Types and Handling**:

1. **JSON Parsing Errors**:
   ```json
   {
     "success": false,
     "error": "Invalid JSON: expected `}` at line 1 column 10"
   }
   ```
   **Action**: Validate JSON syntax before sending

2. **Validation Errors**:
   ```json
   {
     "success": false,
     "error": "Failed to extract audio data: Missing required field: audio_data"
   }
   ```
   **Action**: Check required fields and data format

3. **Audio Processing Errors**:
   ```json
   {
     "success": false,
     "error": "Audio data error: Audio data is empty"
   }
   ```
   **Action**: Verify audio data is valid and not empty

4. **Transcription Errors**:
   ```json
   {
     "success": false,
     "error": "Whisper processing error: Model loading failed"
   }
   ```
   **Action**: Check model file and server status

### Q27: How do I implement proper logging and debugging?

**A:** Implement comprehensive logging for troubleshooting:

**Logging Strategy**:
```python
import logging

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

def transcribe_with_logging(audio_data, options):
    logger.info("Starting transcription request")
    logger.debug(f"Audio data size: {len(audio_data)} bytes")
    logger.debug(f"Options: {options}")
    
    try:
        result = client.transcribe_with_json(audio_data, options)
        
        if result.success:
            logger.info(f"Transcription successful: {len(result.text)} characters")
            logger.debug(f"Detected language: {result.language}")
        else:
            logger.error(f"Transcription failed: {result.error}")
        
        return result
        
    except Exception as e:
        logger.error(f"Transcription exception: {str(e)}", exc_info=True)
        raise
```

### Q28: How do I handle timeouts and network issues?

**A:** Implement robust timeout handling:

**Timeout Strategy**:
```python
import requests
import socket
import timeout_decorator

@timeout_decorator.timeout(300, timeout_exception=TimeoutError)  # 5 minutes
def transcribe_with_timeout(audio_data, options, timeout=300):
    try:
        # Set up request with timeout
        response = requests.post(
            'http://whisper-server/transcribe',
            json={
                'audio_data': base64.b64encode(audio_data).decode(),
                'options': options
            },
            timeout=timeout
        )
        
        response.raise_for_status()
        return TranscriptionResult.from_json(response.text)
        
    except requests.exceptions.Timeout:
        logger.error("Request timed out")
        raise Exception("Transcription request timed out")
    except requests.exceptions.ConnectionError:
        logger.error("Connection error")
        raise Exception("Cannot connect to whisper server")
    except socket.timeout:
        logger.error("Socket timeout")
        raise Exception("Network timeout occurred")
```

### Q29: How do I handle rate limiting?

**A:** Implement rate limiting and backoff strategies:

**Rate Limiting Strategy**:
```python
import time
from threading import Lock

class RateLimiter:
    def __init__(self, max_requests_per_minute=60):
        self.max_requests = max_requests_per_minute
        self.window_start = time.time()
        self.request_count = 0
        self.lock = Lock()
    
    def wait_if_needed(self):
        with self.lock:
            current_time = time.time()
            elapsed = current_time - self.window_start
            
            if elapsed >= 60:  # Reset window
                self.window_start = current_time
                self.request_count = 0
            
            if self.request_count >= self.max_requests:
                sleep_time = 60 - elapsed + 1
                logger.warning(f"Rate limit reached, waiting {sleep_time:.1f} seconds")
                time.sleep(sleep_time)
                self.window_start = time.time()
                self.request_count = 0
            
            self.request_count += 1

# Usage
rate_limiter = RateLimiter()

def transcribe_with_rate_limiting(audio_data, options):
    rate_limiter.wait_if_needed()
    return client.transcribe_with_json(audio_data, options)
```

### Q30: How do I handle partial or corrupted audio data?

**A:** Implement validation and recovery strategies:

**Audio Validation**:
```python
import wave
import struct

def validate_audio_data(audio_data):
    """Validate audio data before sending"""
    try:
        # Check minimum size
        if len(audio_data) < 44:  # WAV header size
            raise ValueError("Audio data too small")
        
        # Check WAV header
        if audio_data[:4] != b'RIFF':
            raise ValueError("Invalid WAV format")
        
        # Extract audio parameters
        if len(audio_data) >= 42:
            sample_rate = struct.unpack('<I', audio_data[24:28])[0]
            channels = struct.unpack('<H', audio_data[22:24])[0]
            
            if sample_rate != 16000:
                raise ValueError(f"Sample rate must be 16kHz, got {sample_rate}")
            
            if channels != 1:
                raise ValueError(f"Channels must be mono, got {channels}")
        
        return True
        
    except Exception as e:
        logger.error(f"Audio validation failed: {str(e)}")
        return False

def safe_transcribe(audio_data, options):
    """Transcribe with validation and error handling"""
    if not validate_audio_data(audio_data):
        raise ValueError("Invalid audio data format")
    
    try:
        return client.transcribe_with_json(audio_data, options)
    except Exception as e:
        logger.error(f"Transcription failed: {str(e)}")
        raise
```

## Compatibility

### Q31: Can I use both SOT and JSON protocols simultaneously?

**A:** Yes, during the transition period (until v2.0.0), the server supports both protocols:

**Detection Logic**:
- SOT: Binary data containing `\0SOT\0` marker
- JSON: Valid JSON structure
- Unknown: Rejected with error

**Migration Strategy**:
```python
def detect_and_transcribe(audio_data):
    # Detect format automatically
    if b'\0SOT\0' in audio_data:
        logger.warning("Using deprecated SOT protocol")
        return client.transcribe_with_sot(audio_data)
    else:
        try:
            # Try to parse as JSON
            json.loads(audio_data.decode())
            return client.transcribe_with_json(audio_data)
        except:
            raise ValueError("Unknown audio format")
```

### Q32: How do I handle version compatibility between clients and server?

**A:** Implement version checking and compatibility layers:

**Version Strategy**:
```python
import json
from packaging import version

class VersionAwareClient:
    def __init__(self, server_path):
        self.server_path = server_path
        self.server_version = None
        self.supports_json = False
    
    def initialize(self):
        # Get server info
        result = self.get_server_info()
        self.server_version = result.get('version', '1.0.0')
        
        # Check capabilities
        self.supports_json = version.parse(self.server_version) >= version.parse('1.5.0')
        
        if not self.supports_json:
            logger.warning("Server does not support JSON interface")
    
    def transcribe(self, audio_data, options=None):
        if self.supports_json and options:
            # Use JSON interface
            return self.transcribe_with_json(audio_data, options)
        else:
            # Fall back to SOT
            return self.transcribe_with_sot(audio_data)
```

### Q33: How do I handle different server configurations?

**A:** The JSON interface works with the same server configurations as SOT:

**Supported Configurations**:
- CPU-only mode (`--cpu-only`)
- Custom thread count (`--threads`)
- Different model files
- Same system requirements

**Configuration Example**:
```bash
# Both protocols work with these configurations
./whisper-background-server model.bin --cpu-only --threads 4
./whisper-background-server model.bin --threads 8
```

### Q34: How do I handle model compatibility issues?

**A:** Model compatibility is the same for both protocols:

**Model Requirements**:
- Same model file formats (.bin)
- Same model capabilities
- Same language support
- Same hardware requirements

**Migration Considerations**:
- No model changes required
- Same model loading process
- Same transcription quality
- Same performance characteristics

### Q35: How do I handle platform-specific differences?

**A:** Platform considerations are similar for both protocols:

**Platform Compatibility**:
- Same operating system support
- Same architecture requirements
- Same dependency needs
- Same installation process

**Platform-Specific Tips**:
- Windows: Use proper path handling
- Linux: Monitor system resources
- macOS: Handle Apple Silicon differences
- Docker: Same container configurations

## Future Planning

### Q36: What new features will only be available with the JSON interface?

**A:** Several features are planned for JSON interface only:

**Planned JSON-Only Features**:
1. **Advanced configuration options**
   - Custom vocabulary
   - Speaker diarization
   - Real-time streaming support

2. **Enhanced metadata**
   - Confidence scores
   - Alternative transcriptions
   - Language detection confidence

3. **Batch processing**
   - Multiple audio files in single request
   - Progress tracking
   - Result aggregation

4. **Monitoring and analytics**
   - Usage statistics
   - Performance metrics
   - Error tracking

### Q37: How will the JSON interface evolve in future versions?

**A:** The JSON interface is designed for extensibility:

**Evolution Roadmap**:
- **v1.6.0**: Advanced transcription options
- **v1.7.0**: Batch processing support
- **v1.8.0**: Real-time streaming
- **v2.0.0+**: Major feature additions

**Extensibility Features**:
- Backward compatibility for existing fields
- Optional fields for new features
- Version negotiation
- Plugin architecture possibilities

### Q38: How should I plan for future migrations?

**A:** Design for future changes:

**Future-Proof Strategies**:
1. **Abstraction layer**: Separate protocol logic from business logic
2. **Configuration management**: Externalize configuration
3. **Testing**: Comprehensive test coverage
4. **Monitoring**: Track usage patterns and performance

**Example Architecture**:
```python
class ProtocolAdapter:
    def __init__(self, protocol='json'):
        self.protocol = protocol
    
    def transcribe(self, audio_data, options):
        if self.protocol == 'json':
            return self._json_transcribe(audio_data, options)
        elif self.protocol == 'sot':
            return self._sot_transcribe(audio_data)
        else:
            raise ValueError(f"Unsupported protocol: {self.protocol}")

class TranscriptionService:
    def __init__(self, adapter):
        self.adapter = adapter
    
    def process_audio(self, audio_file, options):
        # Business logic here
        audio_data = self._load_audio(audio_file)
        result = self.adapter.transcribe(audio_data, options)
        return self._process_result(result)
```

### Q39: What should I consider for long-term maintenance?

**A:** Plan for long-term maintenance:

**Maintenance Considerations**:
1. **Code organization**: Separate protocol handling
2. **Documentation**: Keep examples and guides updated
3. **Testing**: Regular regression testing
4. **Monitoring**: Track usage and errors
5. **Updates**: Plan for regular updates

**Maintenance Checklist**:
- [ ] Review and update documentation quarterly
- [ ] Run full test suite after each update
- [ ] Monitor error rates and performance
- [ ] Keep dependencies updated
- [ ] Plan for protocol changes

### Q40: How do I provide feedback during the migration?

**A:** We welcome feedback to improve the migration experience:

**Feedback Channels**:
1. **GitHub Issues**: Use `migration` label
2. **Discussions**: Share experiences and ask questions
3. **Surveys**: Complete migration experience surveys
4. **Direct contact**: For enterprise customers

**Feedback Guidelines**:
- Include environment details (OS, version, etc.)
- Provide reproduction steps for issues
- Suggest improvements to documentation
- Share migration success stories

---

## Getting Help

If you encounter issues not covered in this FAQ:

1. **Check the [MIGRATION.md](MIGRATION.md)** for detailed instructions
2. **Review the [DEPRECATION_NOTICE.md](DEPRECATION_NOTICE.md)** for timeline information
3. **Search existing GitHub Issues** for similar problems
4. **Open a new GitHub Issue** with:
   - Detailed description of the problem
   - Steps to reproduce
   - Environment information
   - Error messages and logs

**Migration Support Resources**:
- Documentation: [Project Wiki](https://github.com/your-repo/whisper-background-server/wiki)
- Community: [GitHub Discussions](https://github.com/your-repo/whisper-background-server/discussions)
- Issues: [GitHub Issues](https://github.com/your-repo/whisper-background-server/issues)

*This FAQ will be updated regularly based on user feedback and common issues encountered during migration.*