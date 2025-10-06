#!/bin/bash

# Test script for error handling scenarios

echo "=== Testing Error Handling ==="
echo

# Check if large_files directory exists and contains the required files
if [ ! -f "../large_files/ggml-base.en.bin" ]; then
    echo "Error: ../large_files/ggml-base.en.bin not found. Please ensure the large_files directory contains the Whisper model."
    exit 1
fi

# Create test files
mkdir -p test_output/subdir

echo "1. Testing invalid model file (corrupted):"
echo "Command: echo 'corrupted content' > test_output/corrupted_model.bin && echo 'test' | ../target/debug/whisper-background-server test_output/corrupted_model.bin"
echo "Output:"
echo 'corrupted content' > test_output/corrupted_model.bin
(echo 'test' | ../target/debug/whisper-background-server test_output/corrupted_model.bin 2>&1 | head -5)
echo "Exit code: $?"
echo

echo "2. Testing empty model file:"
echo "Command: echo '' > test_output/empty_model.bin && echo 'test' | ../target/debug/whisper-background-server test_output/empty_model.bin"
echo "Output:"
echo '' > test_output/empty_model.bin
(echo 'test' | ../target/debug/whisper-background-server test_output/empty_model.bin 2>&1 | head -5)
echo "Exit code: $?"
echo

echo "3. Testing directory instead of model file:"
echo "Command: mkdir test_output/directory_model && echo 'test' | ../target/debug/whisper-background-server test_output/directory_model"
echo "Output:"
mkdir test_output/directory_model
(echo 'test' | ../target/debug/whisper-background-server test_output/directory_model 2>&1 | head -5)
echo "Exit code: $?"
echo

echo "4. Testing non-existent model file:"
echo "Command: echo 'test' | ../target/debug/whisper-background-server /nonexistent/path/model.bin"
echo "Output:"
(echo 'test' | ../target/debug/whisper-background-server /nonexistent/path/model.bin 2>&1 | head -5)
echo "Exit code: $?"
echo

echo "5. Testing invalid threads value (negative):"
echo "Command: echo 'test' | ./target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads -1"
echo "Output:"
(echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads -1 2>&1 | head -5)
echo "Exit code: $?"
echo

echo "6. Testing invalid threads value (too large):"
echo "Command: echo 'test' | ./target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 999"
echo "Output:"
(echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 999 2>&1 | head -5)
echo "Exit code: $?"
echo

echo "7. Testing unknown argument:"
echo "Command: echo 'test' | ./target/debug/whisper-background-server ../large_files/ggml-base.en.bin --unknown-flag"
echo "Output:"
(echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --unknown-flag 2>&1 | head -5)
echo "Exit code: $?"
echo

echo "8. Testing missing model path argument:"
echo "Command: ../target/debug/whisper-background-server"
echo "Output:"
../target/debug/whisper-background-server 2>&1 | head -5
echo "Exit code: $?"
echo

echo "9. Testing with binary audio data that causes processing errors:"
echo "Command: printf '\xff\xfe\xfd\xfc' | ./target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Output:"
(echo -e '\xff\xfe\xfd\xfc' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 2>&1 | head -5)
echo "Exit code: $?"
echo

echo "10. Testing with SOT marker but no audio data:"
echo "Command: printf '\0SOT\0' | ./target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Output:"
(echo -e '\0SOT\0' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 2>&1 | head -5)
echo "Exit code: $?"
echo

# Clean up
rm -rf test_output/corrupted_model.bin test_output/empty_model.bin test_output/directory_model test_output/subdir

echo "11. Testing with real audio file (jfk.wav) but invalid model:"
echo "Command: base64 ../large_files/jfk.wav | ./target/debug/whisper-background-server /nonexistent/model.bin"
echo "Output:"
(base64 -i ../large_files/jfk.wav | ../target/debug/whisper-background-server /nonexistent/model.bin 2>&1 | head -5)
echo "Exit code: $?"
echo

echo "12. Testing with real audio file (jfk.wav) but invalid threads:"
echo "Command: base64 ../large_files/jfk.wav | ./target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads -1"
echo "Output:"
(base64 -i ../large_files/jfk.wav | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads -1 2>&1 | head -5)
echo "Exit code: $?"
echo

echo "=== Error handling tests completed ==="