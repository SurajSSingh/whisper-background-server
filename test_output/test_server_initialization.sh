#!/bin/bash

# Test script for server initialization with different model configurations

echo "=== Testing Server Initialization ==="
echo

# Check if large_files directory exists and contains the required files
if [ ! -f "../large_files/ggml-base.en.bin" ]; then
    echo "Error: ../large_files/ggml-base.en.bin not found. Please ensure the large_files directory contains the Whisper model."
    exit 1
fi

# Create test files
mkdir -p test_output/subdir
echo -n "" > test_output/empty_model.bin
touch test_output/directory_only

echo "1. Testing valid model file:"
echo "Command: echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Output (first 5 lines):"
(echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 2>&1 | head -5)
echo "Exit code: $?"
echo

echo "2. Testing with threads configuration:"
echo "Command: echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 2"
echo "Output (first 5 lines):"
(echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 2 2>&1 | head -5)
echo "Exit code: $?"
echo

echo "3. Testing with CPU-only configuration:"
echo "Command: echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --cpu-only"
echo "Output (first 5 lines):"
(echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --cpu-only 2>&1 | head -5)
echo "Exit code: $?"
echo

echo "4. Testing with both threads and CPU-only:"
echo "Command: echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 4 --cpu-only"
echo "Output (first 5 lines):"
(echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 4 --cpu-only 2>&1 | head -5)
echo "Exit code: $?"
echo

echo "5. Testing with empty model file:"
echo "Command: echo 'test' | ../target/debug/whisper-background-server test_output/empty_model.bin"
echo "Output (first 5 lines):"
(echo 'test' | ../target/debug/whisper-background-server test_output/empty_model.bin 2>&1 | head -5)
echo "Exit code: $?"
echo

echo "6. Testing with directory instead of file:"
echo "Command: echo 'test' | ../target/debug/whisper-background-server test_output/directory_only"
echo "Output (first 5 lines):"
(echo 'test' | ../target/debug/whisper-background-server test_output/directory_only 2>&1 | head -5)
echo "Exit code: $?"
echo

echo "7. Testing with wrong file extension:"
echo "Command: echo 'test' | ../target/debug/whisper-background-server test_output/valid_model.txt"
echo "Output (first 5 lines):"
(echo 'test' | ../target/debug/whisper-background-server test_output/valid_model.txt 2>&1 | head -5)
echo "Exit code: $?"
echo

echo "8. Testing with nested path:"
echo "Command: echo 'test' | ../target/debug/whisper-background-server test_output/subdir/nested_model.bin"
echo "Output (first 5 lines):"
(echo 'test' | ../target/debug/whisper-background-server test_output/subdir/nested_model.bin 2>&1 | head -5)
echo "Exit code: $?"
echo

# Clean up
rm -rf test_output/empty_model.bin test_output/directory_only test_output/subdir

echo "9. Testing server initialization with real audio file (jfk.wav):"
echo "Command: base64 -i ../large_files/jfk.wav | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Output (first 10 lines):"
(base64 -i ../large_files/jfk.wav | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "10. Testing server initialization with real audio file (jfk.wav) and threads:"
echo "Command: base64 -i ../large_files/jfk.wav | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 4"
echo "Output (first 10 lines):"
(base64 -i ../large_files/jfk.wav | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 4 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "11. Testing server initialization with real audio file (jfk.wav) and CPU-only:"
echo "Command: base64 -i ../large_files/jfk.wav | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --cpu-only"
echo "Output (first 10 lines):"
(base64 -i ../large_files/jfk.wav | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --cpu-only 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "=== Server initialization tests completed ==="