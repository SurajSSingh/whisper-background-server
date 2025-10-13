## ADDED Requirements

### Requirement: JSON Input Interface
The system SHALL accept audio transcription requests via JSON payloads on stdin instead of binary audio data with SOT markers.

#### Scenario: Successful JSON transcription request
- **GIVEN** the server is initialized and ready for input
- **WHEN** a valid JSON payload is received on stdin containing base64-encoded audio data
- **THEN** the system SHALL parse the JSON and extract the audio data
- **AND** the system SHALL validate the JSON structure and required fields
- **AND** the system SHALL trigger the transcription pipeline
- **AND** the system SHALL return the transcription result in the existing JSON format on stdout

#### Scenario: JSON with binary stream audio data
- **GIVEN** the server is initialized and ready for input
- **WHEN** a valid JSON payload is received on stdin containing binary audio data as Vec<u8>
- **THEN** the system SHALL parse the JSON and extract the binary audio data
- **AND** the system SHALL validate the audio data format
- **AND** the system SHALL trigger the transcription pipeline
- **AND** the system SHALL return the transcription result in the existing JSON format on stdout

### Requirement: Transcription Options Configuration
The system SHALL accept configurable transcription options via the JSON input payload.

#### Scenario: Language specification in JSON
- **GIVEN** a JSON payload with transcription options
- **WHEN** the options field contains a valid language code (e.g., "en", "es", "fr")
- **THEN** the system SHALL use the specified language for transcription
- **AND** the transcribed result SHALL reflect the specified language

#### Scenario: Timestamp configuration in JSON
- **GIVEN** a JSON payload with transcription options
- **WHEN** the options field includes "include_timestamps": true
- **THEN** the transcription result SHALL include timestamp segments
- **AND** the result SHALL be formatted according to the existing TranscriptionOutput structure

#### Scenario: Temperature and beam search configuration
- **GIVEN** a JSON payload with advanced transcription options
- **WHEN** the options field contains temperature and beam search parameters
- **THEN** the system SHALL apply these parameters to the Whisper transcription process
- **AND** the transcription SHALL use the specified sampling strategy

### Requirement: Input Validation
The system SHALL validate JSON input payloads and provide structured error responses for invalid input.

#### Scenario: Invalid JSON format
- **GIVEN** an invalid JSON payload is received on stdin
- **WHEN** the JSON cannot be parsed due to syntax errors
- **THEN** the system SHALL return a structured error response on stdout
- **AND** the error response SHALL include an error code and descriptive message
- **AND** the system SHALL continue running for subsequent requests

#### Scenario: Missing required fields
- **GIVEN** a JSON payload is missing the required "audio_data" field
- **WHEN** the system validates the input structure
- **THEN** the system SHALL return a validation error response
- **AND** the error SHALL specify which fields are missing
- **AND** no transcription SHALL be attempted

#### Scenario: Invalid audio data format
- **GIVEN** a JSON payload contains audio data in an unsupported format
- **WHEN** the system validates the audio data
- **THEN** the system SHALL return a format error response
- **AND** the error SHALL describe the specific format issue
- **AND** no transcription SHALL be attempted

### Requirement: Error Handling
The system SHALL provide comprehensive error handling for JSON processing and transcription failures.

#### Scenario: JSON processing error
- **GIVEN** a JSON payload causes processing errors during validation
- **WHEN** the error occurs during input processing
- **THEN** the system SHALL return a structured error response
- **AND** the error SHALL be logged to stderr
- **AND** the system SHALL remain operational for subsequent requests

#### Scenario: Transcription failure with valid input
- **GIVEN** a valid JSON payload with properly formatted audio data
- **WHEN** the Whisper transcription process fails
- **THEN** the system SHALL return an error response in the existing format
- **AND** the error SHALL include details about the transcription failure
- **AND** the error SHALL be logged to stderr

## MODIFIED Requirements

### Requirement: Audio Processing Pipeline
The existing audio processing SHALL be modified to work with JSON-triggered transcription instead of SOT marker detection.

#### Scenario: JSON-triggered transcription
- **GIVEN** the server is processing JSON input instead of binary chunks
- **WHEN** a complete JSON payload is received and validated
- **THEN** the system SHALL extract audio data directly from the JSON
- **AND** the system SHALL bypass the SOT marker detection logic
- **AND** the system SHALL trigger transcription immediately after validation

#### Scenario: Logging output separation
- **GIVEN** the system is processing JSON input
- **WHEN** logging events occur during transcription
- **THEN** all log messages SHALL be sent to stderr
- **AND** stdout SHALL contain only JSON responses (results or errors)
- **AND** no logging SHALL interfere with JSON output parsing

## REMOVED Requirements

### Requirement: SOT Marker Detection
The system SHALL remove the SOT marker detection functionality as it's replaced by JSON interface.

#### Scenario: SOT marker removal
- **GIVEN** the system has been updated to use JSON interface
- **WHEN** binary data containing SOT markers is received
- **THEN** the system SHALL NOT attempt to detect or process SOT markers
- **AND** the system SHALL treat the data as invalid JSON input
- **AND** the system SHALL return a JSON parsing error response

### Requirement: Chunked Audio Buffer
The system SHALL remove the chunked audio buffer and SOT marker processing logic.

#### Scenario: Audio buffer removal
- **GIVEN** the system has been updated to use JSON interface
- **WHEN** audio data is received in chunks
- **THEN** the system SHALL NOT accumulate chunks in a buffer
- **AND** the system SHALL NOT wait for SOT markers
- **AND** the system SHALL expect complete JSON payloads instead