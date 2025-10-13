## Why
The current Whisper Background Server uses a brittle stdin-based input method that relies on raw audio data followed by a custom SOT (`\0SOT\0`) token to trigger transcription. This approach has several limitations:
- Fragile binary protocol that's difficult to debug and maintain
- No structured way to pass transcription options (language, model selection, output format)
- Poor error handling for malformed input
- Inability to validate input before processing
- Logging interference with stdout JSON output

The application needs a robust, structured JSON interface that can encapsulate audio data along with configurable transcription options, providing better error handling, validation, and extensibility.

## What Changes
- **BREAKING**: Replace stdin-based binary audio chunk processing with JSON payload consumption
- **BREAKING**: Remove SOT marker detection and related audio buffer logic
- **ADDED**: Implement JSON input validation and parsing for structured audio requests
- **ADDED**: Support base64-encoded audio data or binary stream representation as `Vec<u8>`
- **ADDED**: Add configurable transcription options via JSON (language, model selection, output format)
- **ADDED**: Implement comprehensive error handling for malformed JSON and incomplete payloads
- **MODIFIED**: Ensure all logging goes to stderr to prevent interference with stdout JSON output
- **MODIFIED**: Update audio processing pipeline to work with JSON-triggered transcription

## Impact
- **Affected specs**: `audio-input` (new capability needed)
- **Affected code**: `src/main.rs`, `src/audio.rs`, `src/transcription.rs`
- **Breaking changes**: Existing clients using the SOT marker protocol will need to be updated
- **Migration path**: Clear documentation and examples for the new JSON interface