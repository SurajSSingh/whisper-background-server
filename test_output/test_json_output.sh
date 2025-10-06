#!/bin/bash

# Test script for JSON output formatting and stdout/stderr separation

echo "=== Testing JSON Output Formatting and Stderr/Stdout Separation ==="
echo

# Check if large_files directory exists and contains the required files
if [ ! -f "../large_files/ggml-base.en.bin" ]; then
    echo "Error: ../large_files/ggml-base.en.bin not found. Please ensure the large_files directory contains the Whisper model."
    exit 1
fi

echo "1. Testing server info JSON output (should be on stdout):"
echo "Command: echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Stdout (JSON server info):"
(echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin >/tmp/stdout.log 2>/tmp/stderr.log)
cat /tmp/stdout.log | head -5
echo
echo "Stderr (logs):"
cat /tmp/stderr.log | head -5
echo "Exit code: $?"
echo

echo "2. Testing error JSON output (when transcription fails):"
echo "Command: printf 'test\0SOT\0' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Stdout (JSON error result):"
(echo -e 'test\0SOT\0' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin >/tmp/stdout2.log 2>/tmp/stderr2.log)
cat /tmp/stdout2.log | head -5
echo
echo "Stderr (logs):"
cat /tmp/stderr2.log | head -5
echo "Exit code: $?"
echo

echo "3. Testing JSON output with threads configuration:"
echo "Command: printf 'test\0SOT\0' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 2"
echo "Stdout (JSON server info):"
(echo -e 'test\0SOT\0' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 2 >/tmp/stdout3.log 2>/tmp/stderr3.log)
cat /tmp/stdout3.log | head -5
echo
echo "Exit code: $?"
echo

echo "4. Testing JSON output with CPU-only configuration:"
echo "Command: printf 'test\0SOT\0' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --cpu-only"
echo "Stdout (JSON server info):"
(echo -e 'test\0SOT\0' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --cpu-only >/tmp/stdout4.log 2>/tmp/stderr4.log)
cat /tmp/stdout4.log | head -5
echo
echo "Exit code: $?"
echo

echo "5. Testing JSON structure validation:"
echo "Command: echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin | python3 -c 'import sys, json; json.load(sys.stdin); print(\"JSON is valid\")'"
echo "Output:"
(echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin | python3 -c 'import sys, json; json.load(sys.stdin); print("JSON is valid")' 2>&1)
echo "Exit code: $?"
echo

echo "6. Testing that logs go to stderr and JSON to stdout:"
echo "Command: echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 1>/dev/null"
echo "Stderr should contain logs:"
(echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 1>/dev/null 2>&1 | head -3)
echo "Exit code: $?"
echo

# Clean up - no dummy files to clean up since we're using real model
rm -f /tmp/stdout*.log /tmp/stderr*.log

echo "7. Testing JSON output with real audio file (jfk.wav) without SOT marker:"
echo "Command: base64 -i ../large_files/jfk.wav | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Stdout (JSON server info):"
(base64 -i ../large_files/jfk.wav | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin >/tmp/stdout_real.log 2>/tmp/stderr_real.log)
cat /tmp/stdout_real.log | head -5
echo
echo "Stderr (logs):"
cat /tmp/stderr_real.log | head -5
echo "Exit code: $?"
echo

echo "8. Testing JSON output with real audio file (jfk.wav) with SOT marker:"
echo "Command: base64 -i ../large_files/jfk.wav | sed 's/$/\\0SOT\\0/' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Stdout (JSON transcription result):"
(base64 -i ../large_files/jfk.wav | sed 's/$/\\0SOT\\0/' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin >/tmp/stdout_real_sot.log 2>/tmp/stderr_real_sot.log)
cat /tmp/stdout_real_sot.log | head -5
echo
echo "Stderr (logs):"
cat /tmp/stderr_real_sot.log | head -5
echo "Exit code: $?"
echo

echo "9. Testing JSON structure validation with real audio:"
echo "Command: base64 -i ../large_files/jfk.wav | sed 's/$/\\0SOT\\0/' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin | python3 -c 'import sys, json; json.load(sys.stdin); print(\"JSON is valid\")'"
echo "Output:"
(base64 -i ../large_files/jfk.wav | sed 's/$/\\0SOT\\0/' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin | python3 -c 'import sys, json; json.load(sys.stdin); print("JSON is valid")' 2>&1)
echo "Exit code: $?"
echo

# Clean up temporary files
rm -f /tmp/stdout_real*.log /tmp/stderr_real*.log

echo "=== JSON output tests completed ==="