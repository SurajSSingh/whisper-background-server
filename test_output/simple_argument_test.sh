#!/bin/bash

# Simple test script for argument parsing without timeout

echo "=== Testing Argument Parsing (Simple) ==="
echo

# Check if large_files directory exists and contains the required files
if [ ! -f "../large_files/ggml-base.en.bin" ]; then
    echo "Error: ../large_files/ggml-base.en.bin not found. Please ensure the large_files directory contains the Whisper model."
    exit 1
fi

echo "1. Testing minimal arguments (model path only):"
echo "Command: echo 'test audio data' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin &"
echo "Output:"
(echo 'test audio data' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin >/dev/null 2>&1 &)
sleep 1
kill %1 2>/dev/null
echo "Exit code: $?"
echo

echo "2. Testing with threads option:"
echo "Command: echo 'test audio data' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 4 &"
echo "Output:"
(echo 'test audio data' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 4 >/dev/null 2>&1 &)
sleep 1
kill %1 2>/dev/null
echo "Exit code: $?"
echo

echo "3. Testing with CPU-only flag:"
echo "Command: echo 'test audio data' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --cpu-only &"
echo "Output:"
(echo 'test audio data' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --cpu-only >/dev/null 2>&1 &)
sleep 1
kill %1 2>/dev/null
echo "Exit code: $?"
echo

echo "4. Testing error case - no arguments:"
echo "Command: ../target/debug/whisper-background-server"
echo "Output:"
../target/debug/whisper-background-server 2>&1 | head -3
echo "Exit code: $?"
echo

echo "5. Testing error case - invalid model path:"
echo "Command: ../target/debug/whisper-background-server /nonexistent/path/model.bin"
echo "Output:"
../target/debug/whisper-background-server /nonexistent/path/model.bin 2>&1 | head -3
echo "Exit code: $?"
echo

echo "6. Testing error case - invalid threads value:"
echo "Command: ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads invalid"
echo "Output:"
../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads invalid 2>&1 | head -3
echo "Exit code: $?"
echo

echo "7. Testing error case - unknown argument:"
echo "Command: ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --unknown"
echo "Output:"
../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --unknown 2>&1 | head -3
echo "Exit code: $?"
echo

echo "8. Testing with real audio file (jfk.wav):"
echo "Command: base64 -i ../large_files/jfk.wav | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Output (first 10 lines):"
(base64 -i ../large_files/jfk.wav | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "9. Testing real audio file with SOT marker:"
echo "Command: base64 -i ../large_files/jfk.wav | sed 's/$/\\0SOT\\0/' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Output (first 15 lines):"
(base64 -i ../large_files/jfk.wav | sed 's/$/\\0SOT\\0/' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 2>&1 | head -15)
echo "Exit code: $?"
echo

echo "10. Testing real audio file with threads configuration:"
echo "Command: base64 -i ../large_files/jfk.wav | sed 's/$/\\0SOT\\0/' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 4"
echo "Output (first 15 lines):"
(base64 -i ../large_files/jfk.wav | sed 's/$/\\0SOT\\0/' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 4 2>&1 | head -15)
echo "Exit code: $?"
echo

# Clean up - no dummy files to clean up since we're using real model

echo "=== Argument parsing tests completed ==="