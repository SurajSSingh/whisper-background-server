## Why
The current `main.rs` file contains logging and environment argument parsing functionality mixed with the main application logic. This makes the code harder to read, maintain, and test. By extracting these concerns into separate modules, we can improve code organization and follow the single responsibility principle.

## What Changes
- **Extract logging functionality** from `main.rs` into a new `src/logging.rs` module
- **Extract environment/argument parsing functionality** from `main.rs` into a new `src/environment.rs` module
- **Update main.rs** to use the new modules while maintaining the same external interface
- **Preserve all existing functionality** including error handling, logging levels, and argument validation
- **Keep the main function structurally the same** with only module import changes

## Impact
- **Affected specs**: No existing specs need modification as this is an internal refactoring
- **Affected code**: 
  - `src/main.rs`: Will be significantly simplified by removing logging and argument parsing code
  - New files: `src/logging.rs` and `src/environment.rs` will be created
  - No changes to public APIs or external interfaces