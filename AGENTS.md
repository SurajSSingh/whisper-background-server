<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# Whisper Background Server Project

## Project Brief

Whisper Background Server project is designed to run Whisper locally on a user's computer in the background. Other services may connect to the server to do transcriptions. It's primary goal is to abstract away many of the parts of running a transcription service locally.

## Technology

Below lists the main technology used in the project. Included are documentation notes for Docs_rs and Context7.

- Rust (edition 2024, MSRV 1.90.0)
- _Key Crates:_
  - [`whisper-rs`](https://docs.rs/whisper-rs/) (v0.15.1): Whisper.cpp bindings for Rust, the primary Whisper implementation to use currently. Context7: `tazz4843/whisper-rs`
  - [`log`](https://docs.rs/crate/log/) (v0.4.28): Logging crate. Context7: `rust-lang/log`
  - [`tokio`](https://docs.rs/tokio) (v1.47.1): Async Runtime. Context7: `websites/rs_tokio` and `tokio-rs/tokio`
  - [`serde`](https://docs.rs/serde) (v1.0.226): Serializing/Deserializing crate. Context7: `serde-rs/serde` and `serde-rs/json`

_**NOTE: Do not include any new dependencies, but you may suggest for user to add later.**_

## Architecture

### Overview of Process

1. Initialize server
  1. Read options passed in:
    - First item (required): Path to model
    - All other items can be placed in any order
    - Options:
      - `--threads <number>`: Number of threads that can be used
    - Flags:
      - `--cpu-only`: Enforce running on CPU without GPU acceleration
  2. Check path is valid and has correct model (e.g., for whisper-rs, it is `.bin`)
  3. Use path and options to load and initialize the model
  4. Send a JSON-serializable object to Stdout with general program info (e.g., provider, model-name, version, attribute and parameters currently set) [This tells parent the programm has successfully launched]
2. Listen on Stdin for input (consider awaiting when in async)
3. Process input when command received:
  1. Receive data as JSON value, containing audio data and any options
  2. Apply any options to setup the transcription service
  3. Load and run full transcription on audio data using loaded model
  4. Extract textual info from transcription result, format into a JSON-serializable object
  5. Send object to Stdout when finished
4. Close server
  1. Kill command from a parent process is given
  2. Rust should automatically clean up the audio buffer and unload the model on drop

### Key Notes

- Standard I/O: This is used to communicate with other processes
  - Stdin: Read from this input (primarily audio data)
  - Stderr: Used to provide log info as needed (may be ignored by listener)
  - Stdout: Output results to this location
- Swapping models: Models cannot change when loaded and the server must be shutdown and be reinitialized with the path to the new model
- Audio data is assumed to be 16 kHz, mono, PCM format
- Use async when possible
- Program arguments (options and flag) have no short variant to prevent confusion
- When to use each logging level (recall that logs go to stderr and may be ignored by client listeners):
  - `error!`: Issues with the app that cannot be recovered from. A client app would need to restart the server.
  - `warn!`: Issues that can be worked around (e.g., have alternative path, use a default value, or ignore request) 
  - `info!`: Checkpoints to know what state the app is currently in. Use to provide overview of step, not specifics.
  - `debug!`: General messages for specific points in a step (consider this as the main alternative to `print!` or `println!`, data should be safe to ignore)
  - `trace!`: Seldomly used, consider for frequently updating data or items which produce large amounts of data

## Common Commnads

- `cargo check`: Check that the current project can compile (use this in-place of build as it is faster)
- `cargo fmt --check`: Check formatting
- `cargo clippy`: Lint current project
- `cargo build`: Build a debug for current target
- `cargo build --release`: Build a release build

### Targets and Features

- `--target=`: Set the build target
- `--feature`/`-f`: Set the features to include as a comma separated value (if multiple given)

## Additional Notes

- Follow OpenSpec (see @/openspec)
- Do not automatically run test
- Do not create integration test unless explictly told by user