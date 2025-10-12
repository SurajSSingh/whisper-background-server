#!/bin/bash

# Test script for JSON interface with real audio files
# This script tests both base64 and binary audio data formats

set -e

echo "=== Testing JSON Interface with Real Audio Files ==="
echo

# Get the absolute path to the model file
MODEL_PATH="$(pwd)/large_files/ggml-base.en.bin"
AUDIO_PATH="$(pwd)/large_files/jfk.wav"

# Check if files exist
if [ ! -f "$MODEL_PATH" ]; then
    echo "Error: Model file not found: $MODEL_PATH"
    exit 1
fi

if [ ! -f "$AUDIO_PATH" ]; then
    echo "Error: Audio file not found: $AUDIO_PATH"
    exit 1
fi

echo "Using model: $MODEL_PATH"
echo "Using audio: $AUDIO_PATH"
echo

# Function to test base64 format
test_base64_format() {
    echo "=== Testing Base64 Format ==="
    
    # Read audio file and encode to base64
    local base64_data=$(base64 -i "$AUDIO_PATH" -w0)
    
    # Create JSON payload with base64 audio data
    local json_payload=$(cat <<EOF
{
    "audio_data": {
        "data": "$base64_data",
        "format": "pcm"
    },
    "options": {
        "language": "en",
        "include_timestamps": true
    }
}
EOF
)
    
    echo "Sending JSON payload with base64 audio data..."
    echo "JSON payload size: ${#json_payload} bytes"
    
    # Test with the server
    echo "$json_payload" | cargo run --release -- --model "$MODEL_PATH" 2>/dev/null || {
        echo "Base64 test failed or server exited"
        return 1
    }
    
    echo "Base64 test completed successfully"
    echo
}

# Function to test binary format
test_binary_format() {
    echo "=== Testing Binary Format ==="
    
    # Read audio file as binary data (simplified approach for testing)
    local binary_data=$(xxd -p "$AUDIO_PATH" | tr -d '\n' | sed 's/\([0-9a-fA-F]\{2\}\)/0x\1,/g')
    
    # Create JSON payload with binary audio data
    local json_payload=$(cat <<EOF
{
    "audio_data": {
        "data": [$binary_data]
    },
    "options": {
        "language": "en",
        "include_timestamps": true
    }
}
EOF
)
    
    echo "Sending JSON payload with binary audio data..."
    echo "JSON payload size: ${#json_payload} bytes"
    
    # Test with the server
    echo "$json_payload" | cargo run --release -- --model "$MODEL_PATH" 2>/dev/null || {
        echo "Binary test failed or server exited"
        return 1
    }
    
    echo "Binary test completed successfully"
    echo
}

# Function to test error handling
test_error_handling() {
    echo "=== Testing Error Handling ==="
    
    # Test with invalid JSON
    echo "Testing invalid JSON..."
    echo "{ invalid json }" | cargo run --release -- --model "$MODEL_PATH" 2>&1 | grep -i "invalid json" || {
        echo "Invalid JSON test failed - expected error message not found"
        return 1
    }
    
    # Test with missing audio data
    echo "Testing missing audio data..."
    echo '{"options": {"language": "en"}}' | cargo run --release -- --model "$MODEL_PATH" 2>&1 | grep -i "audio data" || {
        echo "Missing audio data test failed - expected error message not found"
        return 1
    }
    
    # Test with invalid base64
    echo "Testing invalid base64..."
    echo '{"audio_data": {"data": "invalid_base64!", "format": "pcm"}, "options": {"language": "en"}}' | cargo run --release -- --model "$MODEL_PATH" 2>&1 | grep -i "base64" || {
        echo "Invalid base64 test failed - expected error message not found"
        return 1
    }
    
    echo "Error handling tests completed successfully"
    echo
}

# Function to test logging separation
test_logging_separation() {
    echo "=== Testing Logging Separation ==="
    
    # Create JSON payload
    local base64_data=$(base64 -i "$AUDIO_PATH" -w0)
    local json_payload=$(cat <<EOF
{
    "audio_data": {
        "data": "$base64_data",
        "format": "pcm"
    },
    "options": {
        "language": "en",
        "include_timestamps": true
    }
}
EOF
)
    
    echo "Testing that logs go to stderr and JSON to stdout..."
    
    # Run the server and capture both stdout and stderr
    local result=$(echo "$json_payload" | cargo run --release -- --model "$MODEL_PATH" 2>&1 >/dev/null)
    
    # Check if stderr contains log messages
    if echo "$result" | grep -q "\[.*\]"; then
        echo "✓ Logging found in stderr (as expected)"
    else
        echo "✗ No logging found in stderr"
        return 1
    fi
    
    # Test with direct capture to verify JSON goes to stdout
    local json_output=$(echo "$json_payload" | cargo run --release -- --model "$MODEL_PATH" 2>/dev/null)
    
    if echo "$json_output" | grep -q '"text"'; then
        echo "✓ JSON output found in stdout (as expected)"
    else
        echo "✗ No JSON output found in stdout"
        return 1
    fi
    
    echo "Logging separation test completed successfully"
    echo
}

# Run all tests
echo "Starting comprehensive JSON interface tests..."
echo

test_base64_format
test_binary_format
test_error_handling
test_logging_separation

echo "=== All Tests Completed Successfully ==="
echo "The JSON interface is working correctly with real audio files!"