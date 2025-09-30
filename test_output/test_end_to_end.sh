#!/bin/bash

# Test script for complete end-to-end workflow

echo "=== Testing Complete End-to-End Workflow ==="
echo

# Create a valid model file for testing
echo -n "dummy model content" > test_output/valid_model.bin

echo "1. Testing complete workflow with minimal configuration:"
echo "Command: echo 'test audio data without SOT marker' | ./target/debug/whisper-background-server test_output/valid_model.bin"
echo "Expected: Server loads, waits for audio, but no transcription (no SOT marker)"
echo "Output (first 10 lines):"
(echo 'test audio data without SOT marker' | ./target/debug/whisper-background-server test_output/valid_model.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "2. Testing complete workflow with SOT marker:"
echo "Command: echo -e 'test audio data\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin"
echo "Expected: Server loads, detects SOT marker, attempts transcription"
echo "Output (first 15 lines):"
(echo -e 'test audio data\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin 2>&1 | head -15)
echo "Exit code: $?"
echo

echo "3. Testing complete workflow with threads configuration:"
echo "Command: echo -e 'audio with threads\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin --threads 4"
echo "Expected: Server loads with threads, detects SOT marker, attempts transcription"
echo "Output (first 15 lines):"
(echo -e 'audio with threads\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin --threads 4 2>&1 | head -15)
echo "Exit code: $?"
echo

echo "4. Testing complete workflow with CPU-only configuration:"
echo "Command: echo -e 'audio with cpu only\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin --cpu-only"
echo "Expected: Server loads in CPU mode, detects SOT marker, attempts transcription"
echo "Output (first 15 lines):"
(echo -e 'audio with cpu only\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin --cpu-only 2>&1 | head -15)
echo "Exit code: $?"
echo

echo "5. Testing complete workflow with both options:"
echo "Command: echo -e 'audio with both options\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin --threads 2 --cpu-only"
echo "Expected: Server loads with both options, detects SOT marker, attempts transcription"
echo "Output (first 15 lines):"
(echo -e 'audio with both options\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin --threads 2 --cpu-only 2>&1 | head -15)
echo "Exit code: $?"
echo

echo "6. Testing complete workflow with multiple audio chunks:"
echo "Command: (echo -n 'chunk1\0S' && echo -n 'OT\0chunk2') | ./target/debug/whisper-background-server test_output/valid_model.bin"
echo "Expected: Server loads, processes chunks spanning boundary, detects SOT marker"
echo "Output (first 15 lines):"
((echo -n 'chunk1\0S' && echo -n 'OT\0chunk2') | ./target/debug/whisper-background-server test_output/valid_model.bin 2>&1 | head -15)
echo "Exit code: $?"
echo

echo "7. Testing complete workflow with binary audio data:"
echo "Command: echo -e '\x00\x01\x02audio\x03\x04\0SOT\0\x05\x06' | ./target/debug/whisper-background-server test_output/valid_model.bin"
echo "Expected: Server loads, processes binary data, detects SOT marker"
echo "Output (first 15 lines):"
(echo -e '\x00\x01\x02audio\x03\x04\0SOT\0\x05\x06' | ./target/debug/whisper-background-server test_output/valid_model.bin 2>&1 | head -15)
echo "Exit code: $?"
echo

echo "8. Testing complete workflow with empty audio after SOT marker:"
echo "Command: echo -e 'audio\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin"
echo "Expected: Server loads, detects SOT marker, attempts transcription with empty data"
echo "Output (first 15 lines):"
(echo -e 'audio\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin 2>&1 | head -15)
echo "Exit code: $?"
echo

echo "9. Testing complete workflow with multiple SOT markers:"
echo "Command: echo -e 'audio1\0SOT\0audio2\0SOT\0audio3' | ./target/debug/whisper-background-server test_output/valid_model.bin"
echo "Expected: Server loads, finds last SOT marker, attempts transcription"
echo "Output (first 15 lines):"
(echo -e 'audio1\0SOT\0audio2\0SOT\0audio3' | ./target/debug/whisper-background-server test_output/valid_model.bin 2>&1 | head -15)
echo "Exit code: $?"
echo

echo "10. Testing complete workflow with long audio data:"
echo "Command: echo -e 'this is a longer audio test with multiple words and should trigger the transcription process when the SOT marker is found\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin"
echo "Expected: Server loads, processes long audio, detects SOT marker, attempts transcription"
echo "Output (first 20 lines):"
(echo -e 'this is a longer audio test with multiple words and should trigger the transcription process when the SOT marker is found\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin 2>&1 | head -20)
echo "Exit code: $?"
echo

# Clean up
rm -f test_output/valid_model.bin

echo "=== End-to-end workflow tests completed ==="