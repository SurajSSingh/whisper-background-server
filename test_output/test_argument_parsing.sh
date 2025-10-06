#!/bin/bash

# Test script for argument parsing functionality
# This script tests various command line argument combinations

echo "=== Testing Argument Parsing ==="
echo

# Check if large_files directory exists and contains the required files
if [ ! -f "../large_files/ggml-base.en.bin" ]; then
    echo "Error: ../large_files/ggml-base.en.bin not found. Please ensure the large_files directory contains the Whisper model."
    exit 1
fi

echo "1. Testing minimal arguments (model path only):"
echo "Command: ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
(echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin >/dev/null 2>&1 &) && sleep 1 && kill %1 2>/dev/null 2>&1 | head -5
echo "Exit code: $?"
echo

echo "2. Testing with threads option:"
echo "Command: ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 4"
(echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 4 >/dev/null 2>&1 &) && sleep 1 && kill %1 2>/dev/null 2>&1 | head -5
echo "Exit code: $?"
echo

echo "3. Testing with CPU-only flag:"
echo "Command: ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --cpu-only"
(echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --cpu-only >/dev/null 2>&1 &) && sleep 1 && kill %1 2>/dev/null 2>&1 | head -5
echo "Exit code: $?"
echo

echo "4. Testing with both options:"
echo "Command: ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 8 --cpu-only"
(echo 'test' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 8 --cpu-only >/dev/null 2>&1 &) && sleep 1 && kill %1 2>/dev/null 2>&1 | head -5
echo "Exit code: $?"
echo

echo "5. Testing with invalid model path:"
echo "Command: ../target/debug/whisper-background-server /nonexistent/path/model.bin"
../target/debug/whisper-background-server /nonexistent/path/model.bin 2>&1 | head -5
echo "Exit code: $?"
echo

echo "6. Testing with no arguments:"
echo "Command: ../target/debug/whisper-background-server"
../target/debug/whisper-background-server 2>&1 | head -5
echo "Exit code: $?"
echo

echo "7. Testing with invalid threads value:"
echo "Command: ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads invalid"
timeout 2s ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads invalid 2>&1 | head -5
echo "Exit code: $?"
echo

echo "8. Testing with zero threads:"
echo "Command: ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 0"
timeout 2s ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 0 2>&1 | head -5
echo "Exit code: $?"
echo

echo "9. Testing with unknown argument:"
echo "Command: ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --unknown"
timeout 2s ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --unknown 2>&1 | head -5
echo "Exit code: $?"
echo

echo "10. Testing with threads option but no value:"
echo "Command: ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads"
timeout 2s ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin --threads 2>&1 | head -5
echo "Exit code: $?"
echo

# Clean up - no dummy files to clean up since we're using real model

echo "=== Argument parsing tests completed ==="