#!/bin/bash

# Comprehensive argument parsing test script
# This script tests various argument parsing scenarios for the whisper-background-server

echo "=== Comprehensive Argument Parsing Tests ==="
echo

# Build the project first
echo "Building the project..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Build failed. Exiting."
    exit 1
fi
echo "Build successful."
echo

# Path to the binary
BINARY="./target/release/whisper-background-server"

# Test cases
test_cases=(
    # Valid minimal case
    "Valid minimal case:./large_files/ggml-base.en.bin"
    
    # Valid with threads
    "Valid with threads:./large_files/ggml-base.en.bin --threads 4"
    
    # Valid with cpu-only flag
    "Valid with cpu-only:./large_files/ggml-base.en.bin --cpu-only"
    
    # Valid with both options
    "Valid with both:./large_files/ggml-base.en.bin --threads 8 --cpu-only"
    
    # Valid with different thread values
    "Valid with 1 thread:./large_files/ggml-base.en.bin --threads 1"
    "Valid with many threads:./large_files/ggml-base.en.bin --threads 32"
    
    # Invalid cases
    "No arguments:"
    "No model path:--threads 4"
    "Invalid threads:./large_files/ggml-base.en.bin --threads invalid"
    "Zero threads:./large_files/ggml-base.en.bin --threads 0"
    "Negative threads:./large_files/ggml-base.en.bin --threads -1"
    "Unknown flag:./large_files/ggml-base.en.bin --unknown"
    "Missing threads value:./large_files/ggml-base.en.bin --threads"
    "Multiple cpu-only flags:./large_files/ggml-base.en.bin --cpu-only --cpu-only"
    
    # Edge cases
    "Empty model path:"
    "Relative path:large_files/ggml-base.en.bin"
    "Long thread number:./large_files/ggml-base.en.bin --threads 999999"
)

# Function to run a test case
run_test() {
    local test_name="$1"
    local test_args="$2"
    
    echo "Testing: $test_name"
    echo "Arguments: $test_args"
    
    # Run the test (without timeout since it's not available on macOS)
    $BINARY $test_args 2>&1 | tee -a /tmp/test_output.log &
    local pid=$!
    
    # Wait a bit for the process to start and potentially exit
    sleep 2
    
    # Check if the process is still running
    if kill -0 $pid 2>/dev/null; then
        # Process is still running, likely waiting for input - kill it
        kill $pid 2>/dev/null
        wait $pid 2>/dev/null
        echo "Result: TIMEOUT (likely waiting for input)"
    else
        # Process has exited, check the exit code
        wait $pid
        local exit_code=$?
        if [ $exit_code -eq 0 ]; then
            echo "Result: SUCCESS"
        else
            echo "Result: FAILED (exit code: $exit_code)"
        fi
    fi
    
    echo "----------------------------------------"
    echo
}

# Run all test cases
for test_case in "${test_cases[@]}"; do
    # Split test case into name and args
    IFS=':' read -r test_name test_args <<< "$test_case"
    run_test "$test_name" "$test_args"
done

echo "=== Test Summary ==="
echo "All argument parsing tests completed."
echo "Full output available in /tmp/test_output.log"

# Check if we have any specific error patterns we want to verify
echo
echo "=== Error Pattern Analysis ==="
echo "Looking for common error patterns in the logs..."

# Check for specific error messages
if grep -q "Model path is required" /tmp/test_output.log; then
    echo "✓ Found 'Model path is required' error handling"
else
    echo "✗ Missing 'Model path is required' error handling"
fi

if grep -q "Unknown argument" /tmp/test_output.log; then
    echo "✓ Found 'Unknown argument' error handling"
else
    echo "✗ Missing 'Unknown argument' error handling"
fi

if grep -q "Invalid number of threads" /tmp/test_output.log; then
    echo "✓ Found 'Invalid number of threads' error handling"
else
    echo "✗ Missing 'Invalid number of threads' error handling"
fi

if grep -q "Number of threads must be greater than 0" /tmp/test_output.log; then
    echo "✓ Found 'Number of threads must be greater than 0' error handling"
else
    echo "✗ Missing 'Number of threads must be greater than 0' error handling"
fi

echo
echo "=== Test Complete ==="