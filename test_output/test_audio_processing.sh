#!/bin/bash

# Test script for audio processing with SOT marker detection

echo "=== Testing Audio Processing with SOT Marker Detection ==="
echo

# Check if large_files directory exists and contains the required files
if [ ! -f "../large_files/ggml-base.en.bin" ]; then
    echo "Error: ../large_files/ggml-base.en.bin not found. Please ensure the large_files directory contains the Whisper model."
    exit 1
fi

echo "1. Testing basic audio processing without SOT marker:"
echo "Command: echo 'test audio data without marker' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Output (first 10 lines):"
(echo 'test audio data without marker' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "2. Testing audio processing with SOT marker at the end:"
echo "Command: printf 'test audio data\0SOT\0' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Output (first 10 lines):"
(echo -e 'test audio data\0SOT\0' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "3. Testing audio processing with SOT marker in the middle:"
echo "Command: printf 'audio before\0SOT\0audio after' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Output (first 10 lines):"
(echo -e 'audio before\0SOT\0audio after' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "4. Testing audio processing with multiple SOT markers (should find the last one):"
echo "Command: printf 'audio\0SOT\0middle\0SOT\0end' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Output (first 10 lines):"
(echo -e 'audio\0SOT\0middle\0SOT\0end' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "5. Testing audio processing with partial SOT marker (should not trigger):"
echo "Command: printf 'audio\0SOT' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Output (first 10 lines):"
(echo -e 'audio\0SOT' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "6. Testing audio processing with SOT marker spanning multiple chunks:"
echo "Command: (echo -n 'part1\0S' && echo -n 'OT\0part2') | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Output (first 10 lines):"
((echo -n 'part1\0S' && echo -n 'OT\0part2') | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "7. Testing with empty audio data:"
echo "Command: echo '' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Output (first 10 lines):"
(echo '' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "8. Testing with binary data that might accidentally contain SOT marker:"
echo "Command: printf '\x00\x01\x02\x00S\x00O\x00T\x00\x03\x04\x05' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Output (first 10 lines):"
(echo -e '\x00\x01\x02\x00S\x00O\x00T\x00\x03\x04\x05' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

# Clean up - no dummy files to clean up since we're using real model

echo "9. Testing with real audio file (jfk.wav) without SOT marker:"
echo "Command: base64 -i ../large_files/jfk.wav | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Output (first 10 lines):"
(base64 -i ../large_files/jfk.wav | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 2>&1 | head -10)
echo "Exit code: $?"
echo

echo "10. Testing with real audio file (jfk.wav) with SOT marker:"
echo "Command: base64 -i ../large_files/jfk.wav | sed 's/$/\\0SOT\\0/' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin"
echo "Output (first 15 lines):"
(base64 -i ../large_files/jfk.wav | sed 's/$/\\0SOT\\0/' | ../target/debug/whisper-background-server ../large_files/ggml-base.en.bin 2>&1 | head -15)
echo "Exit code: $?"
echo

echo "=== Audio processing tests completed ==="