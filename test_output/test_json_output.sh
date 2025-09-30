#!/bin/bash

# Test script for JSON output formatting and stdout/stderr separation

echo "=== Testing JSON Output Formatting and Stderr/Stdout Separation ==="
echo

# Create a valid model file for testing
echo -n "dummy model content" > test_output/valid_model.bin

echo "1. Testing server info JSON output (should be on stdout):"
echo "Command: echo 'test' | ./target/debug/whisper-background-server test_output/valid_model.bin"
echo "Stdout (JSON server info):"
(echo 'test' | ./target/debug/whisper-background-server test_output/valid_model.bin >/tmp/stdout.log 2>/tmp/stderr.log)
cat /tmp/stdout.log | head -5
echo
echo "Stderr (logs):"
cat /tmp/stderr.log | head -5
echo "Exit code: $?"
echo

echo "2. Testing error JSON output (when transcription fails):"
echo "Command: printf 'test\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin"
echo "Stdout (JSON error result):"
(echo -e 'test\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin >/tmp/stdout2.log 2>/tmp/stderr2.log)
cat /tmp/stdout2.log | head -5
echo
echo "Stderr (logs):"
cat /tmp/stderr2.log | head -5
echo "Exit code: $?"
echo

echo "3. Testing JSON output with threads configuration:"
echo "Command: printf 'test\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin --threads 2"
echo "Stdout (JSON server info):"
(echo -e 'test\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin --threads 2 >/tmp/stdout3.log 2>/tmp/stderr3.log)
cat /tmp/stdout3.log | head -5
echo
echo "Exit code: $?"
echo

echo "4. Testing JSON output with CPU-only configuration:"
echo "Command: printf 'test\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin --cpu-only"
echo "Stdout (JSON server info):"
(echo -e 'test\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin --cpu-only >/tmp/stdout4.log 2>/tmp/stderr4.log)
cat /tmp/stdout4.log | head -5
echo
echo "Exit code: $?"
echo

echo "5. Testing JSON structure validation:"
echo "Command: echo 'test' | ./target/debug/whisper-background-server test_output/valid_model.bin | python3 -c 'import sys, json; json.load(sys.stdin); print(\"JSON is valid\")'"
echo "Output:"
(echo 'test' | ./target/debug/whisper-background-server test_output/valid_model.bin | python3 -c 'import sys, json; json.load(sys.stdin); print("JSON is valid")' 2>&1)
echo "Exit code: $?"
echo

echo "6. Testing that logs go to stderr and JSON to stdout:"
echo "Command: echo 'test' | ./target/debug/whisper-background-server test_output/valid_model.bin 1>/dev/null"
echo "Stderr should contain logs:"
(echo 'test' | ./target/debug/whisper-background-server test_output/valid_model.bin 1>/dev/null 2>&1 | head -3)
echo "Exit code: $?"
echo

# Clean up
rm -f test_output/valid_model.bin /tmp/stdout*.log /tmp/stderr*.log

echo "=== JSON output tests completed ==="