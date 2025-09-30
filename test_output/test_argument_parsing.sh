#!/bin/bash

# Test script for argument parsing functionality
# This script tests various command line argument combinations

echo "=== Testing Argument Parsing ==="
echo

# Create a dummy model file for testing
touch test_output/test_model.bin

echo "1. Testing minimal arguments (model path only):"
echo "Command: ./target/debug/whisper-background-server test_output/test_model.bin"
timeout 2s ./target/debug/whisper-background-server test_output/test_model.bin 2>&1 | head -5
echo "Exit code: $?"
echo

echo "2. Testing with threads option:"
echo "Command: ./target/debug/whisper-background-server test_output/test_model.bin --threads 4"
timeout 2s ./target/debug/whisper-background-server test_output/test_model.bin --threads 4 2>&1 | head -5
echo "Exit code: $?"
echo

echo "3. Testing with CPU-only flag:"
echo "Command: ./target/debug/whisper-background-server test_output/test_model.bin --cpu-only"
timeout 2s ./target/debug/whisper-background-server test_output/test_model.bin --cpu-only 2>&1 | head -5
echo "Exit code: $?"
echo

echo "4. Testing with both options:"
echo "Command: ./target/debug/whisper-background-server test_output/test_model.bin --threads 8 --cpu-only"
timeout 2s ./target/debug/whisper-background-server test_output/test_model.bin --threads 8 --cpu-only 2>&1 | head -5
echo "Exit code: $?"
echo

echo "5. Testing with invalid model path:"
echo "Command: ./target/debug/whisper-background-server /nonexistent/path/model.bin"
timeout 2s ./target/debug/whisper-background-server /nonexistent/path/model.bin 2>&1 | head -5
echo "Exit code: $?"
echo

echo "6. Testing with no arguments:"
echo "Command: ./target/debug/whisper-background-server"
timeout 2s ./target/debug/whisper-background-server 2>&1 | head -5
echo "Exit code: $?"
echo

echo "7. Testing with invalid threads value:"
echo "Command: ./target/debug/whisper-background-server test_output/test_model.bin --threads invalid"
timeout 2s ./target/debug/whisper-background-server test_output/test_model.bin --threads invalid 2>&1 | head -5
echo "Exit code: $?"
echo

echo "8. Testing with zero threads:"
echo "Command: ./target/debug/whisper-background-server test_output/test_model.bin --threads 0"
timeout 2s ./target/debug/whisper-background-server test_output/test_model.bin --threads 0 2>&1 | head -5
echo "Exit code: $?"
echo

echo "9. Testing with unknown argument:"
echo "Command: ./target/debug/whisper-background-server test_output/test_model.bin --unknown"
timeout 2s ./target/debug/whisper-background-server test_output/test_model.bin --unknown 2>&1 | head -5
echo "Exit code: $?"
echo

echo "10. Testing with threads option but no value:"
echo "Command: ./target/debug/whisper-background-server test_output/test_model.bin --threads"
timeout 2s ./target/debug/whisper-background-server test_output/test_model.bin --threads 2>&1 | head -5
echo "Exit code: $?"
echo

# Clean up
rm -f test_output/test_model.bin

echo "=== Argument parsing tests completed ==="