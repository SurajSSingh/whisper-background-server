#!/bin/bash

# Simple test script for argument parsing without timeout

echo "=== Testing Argument Parsing (Simple) ==="
echo

# Create a dummy model file for testing
touch test_output/test_model.bin

echo "1. Testing minimal arguments (model path only):"
echo "Command: echo 'test audio data' | ./target/debug/whisper-background-server test_output/test_model.bin &"
echo "Output:"
(echo 'test audio data' | ./target/debug/whisper-background-server test_output/test_model.bin >/dev/null 2>&1 &)
sleep 1
kill %1 2>/dev/null
echo "Exit code: $?"
echo

echo "2. Testing with threads option:"
echo "Command: echo 'test audio data' | ./target/debug/whisper-background-server test_output/test_model.bin --threads 4 &"
echo "Output:"
(echo 'test audio data' | ./target/debug/whisper-background-server test_output/test_model.bin --threads 4 >/dev/null 2>&1 &)
sleep 1
kill %1 2>/dev/null
echo "Exit code: $?"
echo

echo "3. Testing with CPU-only flag:"
echo "Command: echo 'test audio data' | ./target/debug/whisper-background-server test_output/test_model.bin --cpu-only &"
echo "Output:"
(echo 'test audio data' | ./target/debug/whisper-background-server test_output/test_model.bin --cpu-only >/dev/null 2>&1 &)
sleep 1
kill %1 2>/dev/null
echo "Exit code: $?"
echo

echo "4. Testing error case - no arguments:"
echo "Command: ./target/debug/whisper-background-server"
echo "Output:"
./target/debug/whisper-background-server 2>&1 | head -3
echo "Exit code: $?"
echo

echo "5. Testing error case - invalid model path:"
echo "Command: ./target/debug/whisper-background-server /nonexistent/path/model.bin"
echo "Output:"
./target/debug/whisper-background-server /nonexistent/path/model.bin 2>&1 | head -3
echo "Exit code: $?"
echo

echo "6. Testing error case - invalid threads value:"
echo "Command: ./target/debug/whisper-background-server test_output/test_model.bin --threads invalid"
echo "Output:"
./target/debug/whisper-background-server test_output/test_model.bin --threads invalid 2>&1 | head -3
echo "Exit code: $?"
echo

echo "7. Testing error case - unknown argument:"
echo "Command: ./target/debug/whisper-background-server test_output/test_model.bin --unknown"
echo "Output:"
./target/debug/whisper-background-server test_output/test_model.bin --unknown 2>&1 | head -3
echo "Exit code: $?"
echo

# Clean up
rm -f test_output/test_model.bin

echo "=== Argument parsing tests completed ==="