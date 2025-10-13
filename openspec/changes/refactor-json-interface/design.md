## Context
The Whisper Background Server currently uses a binary protocol with SOT markers for audio input processing. This design document outlines the new JSON-based interface that will replace the current brittle stdin-based approach.

## Goals / Non-Goals
### Goals
- Replace brittle SOT marker-based input with robust JSON interface
- Support configurable transcription options via JSON payload
- Provide comprehensive input validation and error handling
- Maintain backward compatibility in JSON output format
- Ensure all logging goes to stderr to prevent interference with stdout JSON
- Support both base64-encoded and binary stream audio data representations

### Non-Goals
- Maintaining backward compatibility with SOT marker protocol
- Supporting multiple simultaneous audio streams
- Real-time streaming transcription (batch processing only)
- Audio format validation beyond basic checks

## Decisions

### JSON Schema Design
The new JSON interface will use the following schema:

```json
{
  "audio_data": "base64_encoded_audio_string_or_binary_stream",
  "options": {
    "language": "en|auto|zh|de|es|ru|ko|fr|ja|pt|tr|pl|ca",
    "translate_to_english": false,
    "include_timestamps": true,
    "max_tokens": 100,
    "temperature": 0.0,
    "use_beam_search": false,
    "beam_size": 5,
    "suppress_blank": true,
    "word_timestamps": false
  }
}
```

### Audio Data Representation
Two supported formats:
1. **Base64-encoded string**: For easy JSON serialization and transport
2. **Binary stream as Vec<u8>**: For direct binary data handling

### Error Handling Strategy
- JSON validation errors return structured error responses
- Audio processing errors maintain current error format
- All error responses include error codes and descriptive messages
- Logging goes exclusively to stderr

### Processing Pipeline
1. Read complete JSON payload from stdin
2. Validate JSON structure and required fields
3. Parse and validate audio data format
4. Extract transcription options
5. Trigger transcription pipeline
6. Return results in existing JSON format

## Risks / Trade-offs
- **Risk**: Base64 encoding increases payload size by ~33%
  - **Mitigation**: Support binary stream format for large audio files
- **Risk**: JSON parsing overhead for real-time applications
  - **Mitigation**: Efficient streaming JSON parser and async processing
- **Risk**: Breaking change for existing clients
  - **Mitigation**: Clear migration documentation and examples

## Migration Plan
1. **Phase 1**: Implement new JSON interface alongside existing SOT protocol
2. **Phase 2**: Add deprecation warnings for SOT protocol
3. **Phase 3**: Remove SOT protocol support in next major version
4. **Documentation**: Provide comprehensive migration guide with examples

## Open Questions
1. Should we support chunked JSON streaming for very large audio files?
2. What's the maximum supported audio file size for JSON interface?
3. Should we add support for additional audio formats beyond 16kHz mono PCM?
4. How should we handle partial JSON payloads (incomplete data)?

### Answers to Questions
1. No, audio file is not chunked
2. No direct size, safe to assume audio data is less than 5 minutes
3. No, assume the other app has done conversion already
4. Incomplete data should send a message back to other app to resend data (override incomplete)