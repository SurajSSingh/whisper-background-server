#!/bin/bash

# Test script for audio processing with SOT marker detection

echo "=== Testing Audio Processing with SOT Marker Detection ==="
echo

# Create a valid model file for testing
echo -n "dummy model content" > test_output/valid_model.bin

echo "1. Testing basic audio processing without SOT marker:"
echo "Command: echo 'test audio data without marker' | ./target/debug/whisper-background-server test_output/valid_model.bin"
echo "Output (first 10 lines):"
(echo 'test audio data without marker' | ./target/debug/whisper-background-server test_output/valid_model.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "2. Testing audio processing with SOT marker at the end:"
echo "Command: printf 'test audio data\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin"
echo "Output (first 10 lines):"
(echo -e 'test audio data\0SOT\0' | ./target/debug/whisper-background-server test_output/valid_model.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "3. Testing audio processing with SOT marker in the middle:"
echo "Command: printf 'audio before\0SOT\0audio after' | ./target/debug/whisper-background-server test_output/valid_model.bin"
echo "Output (first 10 lines):"
(echo -e 'audio before\0SOT\0audio after' | ./target/debug/whisper-background-server test_output/valid_model.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "4. Testing audio processing with multiple SOT markers (should find the last one):"
echo "Command: printf 'audio\0SOT\0middle\0SOT\0end' | ./target/debug/whisper-background-server test_output/valid_model.bin"
echo "Output (first 10 lines):"
(echo -e 'audio\0SOT\0middle\0SOT\0end' | ./target/debug/whisper-background-server test_output/valid_model.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "5. Testing audio processing with partial SOT marker (should not trigger):"
echo "Command: printf 'audio\0SOT' | ./target/debug/whisper-background-server test_output/valid_model.bin"
echo "Output (first 10 lines):"
(echo -e 'audio\0SOT' | ./target/debug/whisper-background-server test_output/valid_model.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "6. Testing audio processing with SOT marker spanning multiple chunks:"
echo "Command: (echo -n 'part1\0S' && echo -n 'OT\0part2') | ./target/debug/whisper-background-server test_output/valid_model.bin"
echo "Output (first 10 lines):"
((echo -n 'part1\0S' && echo -n 'OT\0part2') | ./target/debug/whisper-background-server test_output/valid_model.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "7. Testing with empty audio data:"
echo "Command: echo '' | ./target/debug/whisper-background-server test_output/valid_model.bin"
echo "Output (first 10 lines):"
(echo '' | ./target/debug/whisper-background-server test_output/valid_model.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "8. Testing with binary data that might accidentally contain SOT marker:"
echo "Command: printf '\x00\x01\x02\x00S\x00O\x00T\x00\x03\x04\x05' | ./target/debug/whisper-background-server test_output/valid_model.bin"
echo "Output (first 10 lines):"
(echo -e '\x00\x01\x02\x00S\x00O\x00T\x00\x03\x04\x05' | ./target/debug/whisper-background-server test_output/valid_model.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

# Clean up
rm -f test_output/valid_model.bin

echo "=== Audio processing tests completed ==="