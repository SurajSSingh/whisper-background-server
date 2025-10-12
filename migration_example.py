#!/usr/bin/env python3
"""
Migration Example: SOT to JSON Interface Converter

This script demonstrates how to migrate from the SOT protocol to the JSON interface
for the Whisper Background Server. It includes examples of both conversion utilities
and client migration patterns.

Usage:
    python migration_example.py --help
    python migration_example.py convert --input audio.sot --output audio.json
    python migration_example.py client --mode basic
    python migration_example.py test --model model.bin
"""

import argparse
import base64
import json
import logging
import os
import subprocess
import sys
import tempfile
import time
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Optional, Dict, Any, List, Union
from enum import Enum

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

class AudioFormat(Enum):
    """Supported audio data formats"""
    BASE64 = "base64"
    BINARY = "binary"

class InputFormat(Enum):
    """Input format detection"""
    SOT = "sot"
    JSON = "json"
    UNKNOWN = "unknown"

@dataclass
class TranscriptionOptions:
    """Transcription options for JSON interface"""
    language: Optional[str] = None
    translate_to_english: Optional[bool] = False
    include_timestamps: Optional[bool] = True
    max_tokens: Optional[int] = None
    temperature: Optional[float] = 0.0
    use_beam_search: Optional[bool] = False
    beam_size: Optional[int] = None
    suppress_blank: Optional[bool] = True
    word_timestamps: Optional[bool] = False

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for JSON serialization"""
        return {k: v for k, v in asdict(self).items() if v is not None}

@dataclass
class TranscriptionResult:
    """Transcription result from server"""
    text: str
    language: Optional[str] = None
    segments: Optional[List[Dict[str, Any]]] = None
    success: bool = True
    error: Optional[str] = None
    duration_ms: Optional[int] = None
    timestamp: Optional[str] = None

    @classmethod
    def from_json(cls, json_str: str) -> 'TranscriptionResult':
        """Create from JSON response"""
        data = json.loads(json_str)
        return cls(**data)

class SOTProcessor:
    """Processor for SOT protocol data"""
    
    SOT_MARKER = b'\0SOT\0'
    
    @classmethod
    def has_sot_marker(cls, data: bytes) -> bool:
        """Check if data contains SOT marker"""
        return cls.SOT_MARKER in data
    
    @classmethod
    def extract_audio_from_sot(cls, data: bytes) -> bytes:
        """Extract audio data before SOT marker"""
        if not cls.has_sot_marker(data):
            raise ValueError("No SOT marker found in data")
        
        # Find SOT marker position
        sot_pos = data.find(cls.SOT_MARKER)
        return data[:sot_pos]
    
    @classmethod
    def convert_sot_to_json_request(
        cls, 
        audio_data: bytes, 
        options: Optional[TranscriptionOptions] = None
    ) -> Dict[str, Any]:
        """Convert SOT audio data to JSON request format"""
        # Extract audio data before SOT marker
        extracted_audio = cls.extract_audio_from_sot(audio_data)
        
        # Create base64-encoded audio data
        base64_audio = base64.b64encode(extracted_audio).decode('utf-8')
        
        # Build JSON request
        request = {
            "audio_data": {
                "data": base64_audio,
                "format": "wav"
            }
        }
        
        if options:
            request["options"] = options.to_dict()
        
        return request

class CompatibilityLayer:
    """Compatibility layer for gradual migration"""
    
    @staticmethod
    def detect_input_format(data: Union[str, bytes]) -> InputFormat:
        """Detect input format (SOT or JSON)"""
        if isinstance(data, str):
            data = data.encode('utf-8')
        
        # Check for SOT marker first
        if SOTProcessor.has_sot_marker(data):
            return InputFormat.SOT
        
        # Try to parse as JSON
        try:
            json.loads(data.decode('utf-8'))
            return InputFormat.JSON
        except (json.JSONDecodeError, UnicodeDecodeError):
            return InputFormat.UNKNOWN
    
    @staticmethod
    def convert_sot_to_json_request(
        audio_data: bytes, 
        options: Optional[TranscriptionOptions] = None
    ) -> Dict[str, Any]:
        """Convert SOT data to JSON request"""
        return SOTProcessor.convert_sot_to_json_request(audio_data, options)
    
    @staticmethod
    def convert_json_to_sot_like(request: Dict[str, Any]) -> bytes:
        """Convert JSON request to SOT-like binary data"""
        audio_data = request["audio_data"]["data"]
        
        if isinstance(audio_data, str):
            # Base64 encoded data
            audio_bytes = base64.b64decode(audio_data)
        else:
            # Binary data as list
            audio_bytes = bytes(audio_data)
        
        # Add SOT marker
        return audio_bytes + SOTProcessor.SOT_MARKER

class WhisperClient:
    """Client for Whisper Background Server with both SOT and JSON support"""
    
    def __init__(self, server_path: str = "./whisper-background-server"):
        self.server_path = server_path
        self.server_process = None
    
    def start_server(self, model_path: str, **kwargs) -> subprocess.Popen:
        """Start the whisper server"""
        cmd = [self.server_path, model_path]
        
        # Add optional parameters
        if 'threads' in kwargs:
            cmd.extend(['--threads', str(kwargs['threads'])])
        if kwargs.get('cpu_only', False):
            cmd.append('--cpu-only')
        
        logger.info(f"Starting server with command: {' '.join(cmd)}")
        
        self.server_process = subprocess.Popen(
            cmd,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        
        # Wait for server initialization
        stdout, stderr = self.server_process.communicate(timeout=10)
        
        if self.server_process.returncode != 0:
            raise RuntimeError(f"Server failed to start: {stderr}")
        
        try:
            server_info = json.loads(stdout)
            logger.info(f"Server started: {server_info.get('model_name', 'unknown')}")
            return self.server_process
        except json.JSONDecodeError:
            logger.warning("Server started but no valid JSON info received")
            return self.server_process
    
    def stop_server(self):
        """Stop the whisper server"""
        if self.server_process:
            self.server_process.terminate()
            self.server_process.wait(timeout=5)
            self.server_process = None
    
    def transcribe_with_sot(self, audio_data: bytes) -> str:
        """Transcribe using SOT protocol (legacy)"""
        if not self.server_process:
            raise RuntimeError("Server not started")
        
        try:
            # Send audio data with SOT marker
            self.server_process.stdin.write(audio_data.decode('latin-1'))
            self.server_process.stdin.write("\0SOT\0")
            self.server_process.stdin.flush()
            
            # Get response
            stdout, stderr = self.server_process.communicate()
            
            if self.server_process.returncode != 0:
                raise RuntimeError(f"Server error: {stderr}")
            
            return stdout.strip()
            
        except Exception as e:
            raise RuntimeError(f"Transcription failed: {str(e)}")
    
    def transcribe_with_json(
        self, 
        audio_data: Union[str, bytes], 
        options: Optional[TranscriptionOptions] = None
    ) -> TranscriptionResult:
        """Transcribe using JSON interface"""
        if not self.server_process:
            raise RuntimeError("Server not started")
        
        try:
            # Prepare request
            if isinstance(audio_data, bytes):
                # Convert bytes to base64
                audio_b64 = base64.b64encode(audio_data).decode('utf-8')
            else:
                # Assume it's already base64 or file path
                audio_b64 = audio_data
            
            request = {
                "audio_data": {
                    "data": audio_b64,
                    "format": "wav"
                }
            }
            
            if options:
                request["options"] = options.to_dict()
            
            json_request = json.dumps(request)
            
            # Send request
            self.server_process.stdin.write(json_request)
            self.server_process.stdin.flush()
            
            # Get response
            stdout, stderr = self.server_process.communicate()
            
            if self.server_process.returncode != 0:
                raise RuntimeError(f"Server error: {stderr}")
            
            # Parse JSON response
            return TranscriptionResult.from_json(stdout)
            
        except Exception as e:
            raise RuntimeError(f"Transcription failed: {str(e)}")
    
    def __enter__(self):
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        self.stop_server()

class MigrationConverter:
    """Utility for converting between SOT and JSON formats"""
    
    @staticmethod
    def convert_file(
        input_path: str,
        output_path: str,
        output_format: str = "json",
        options: Optional[TranscriptionOptions] = None
    ) -> bool:
        """Convert file from SOT to JSON or vice versa"""
        try:
            # Read input file
            with open(input_path, 'rb') as f:
                input_data = f.read()
            
            # Detect input format
            input_format = CompatibilityLayer.detect_input_format(input_data)
            
            if input_format == InputFormat.UNKNOWN:
                logger.error(f"Unknown input format for file: {input_path}")
                return False
            
            logger.info(f"Converting {input_format.value} to {output_format}")
            
            # Convert data
            if output_format == "json":
                if input_format == InputFormat.SOT:
                    request = CompatibilityLayer.convert_sot_to_json_request(input_data, options)
                    output_data = json.dumps(request, indent=2).encode('utf-8')
                else:
                    # Already JSON, just reformat
                    request = json.loads(input_data.decode('utf-8'))
                    output_data = json.dumps(request, indent=2).encode('utf-8')
            elif output_format == "sot":
                request = json.loads(input_data.decode('utf-8'))
                output_data = CompatibilityLayer.convert_json_to_sot_like(request)
            else:
                logger.error(f"Unsupported output format: {output_format}")
                return False
            
            # Write output file
            with open(output_path, 'wb') as f:
                f.write(output_data)
            
            logger.info(f"Successfully converted {input_path} to {output_path}")
            return True
            
        except Exception as e:
            logger.error(f"Conversion failed: {str(e)}")
            return False
    
    @staticmethod
    def convert_stdin_to_stdout(output_format: str = "json") -> bool:
        """Convert from stdin to stdout"""
        try:
            # Read from stdin
            input_data = sys.stdin.buffer.read()
            
            # Detect input format
            input_format = CompatibilityLayer.detect_input_format(input_data)
            
            if input_format == InputFormat.UNKNOWN:
                logger.error("Unknown input format from stdin")
                return False
            
            logger.info(f"Converting {input_format.value} to {output_format}")
            
            # Convert data
            if output_format == "json":
                if input_format == InputFormat.SOT:
                    request = CompatibilityLayer.convert_sot_to_json_request(input_data)
                    output_data = json.dumps(request, indent=2).encode('utf-8')
                else:
                    # Already JSON, just reformat
                    output_data = input_data
            elif output_format == "sot":
                request = json.loads(input_data.decode('utf-8'))
                output_data = CompatibilityLayer.convert_json_to_sot_like(request)
            else:
                logger.error(f"Unsupported output format: {output_format}")
                return False
            
            # Write to stdout
            sys.stdout.buffer.write(output_data)
            sys.stdout.buffer.flush()
            
            return True
            
        except Exception as e:
            logger.error(f"Conversion failed: {str(e)}")
            return False

def create_test_audio_data() -> bytes:
    """Create simple test audio data (silence)"""
    # Create a simple WAV file header + silence
    # This is just for testing purposes
    header = (
        b'RIFF' +  # RIFF header
        (36).to_bytes(4, 'little') +  # File size - 8
        b'WAVE' +  # WAVE header
        b'fmt ' +  # Format chunk
        (16).to_bytes(4, 'little') +  # Format chunk size
        (1).to_bytes(2, 'little') +  # Audio format (1 = PCM)
        (1).to_bytes(2, 'little') +  # Number of channels (1 = mono)
        (16000).to_bytes(4, 'little') +  # Sample rate
        (32000).to_bytes(4, 'little') +  # Byte rate (sample rate * block align)
        (2).to_bytes(2, 'little') +  # Block align (channels * bits per sample / 8)
        (16).to_bytes(2, 'little') +  # Bits per sample
        b'data' +  # Data chunk
        (1000).to_bytes(4, 'little')  # Data size
    )
    
    # Add silence (16-bit samples at 16kHz)
    silence = bytes([0, 0] * 500)  # 500 samples of silence
    
    return header + silence

def run_basic_test():
    """Run basic functionality test"""
    logger.info("Running basic migration test...")
    
    # Create test audio data with SOT marker
    test_audio = create_test_audio_data()
    sot_data = test_audio + SOTProcessor.SOT_MARKER
    
    # Test SOT to JSON conversion
    logger.info("Testing SOT to JSON conversion...")
    request = SOTProcessor.convert_sot_to_json_request(sot_data)
    logger.info(f"Generated JSON request: {json.dumps(request, indent=2)[:200]}...")
    
    # Test JSON to SOT conversion
    logger.info("Testing JSON to SOT conversion...")
    sot_like_data = CompatibilityLayer.convert_json_to_sot_like(request)
    assert SOTProcessor.has_sot_marker(sot_like_data)
    logger.info("SOT marker preserved in conversion")
    
    # Test format detection
    logger.info("Testing format detection...")
    assert CompatibilityLayer.detect_input_format(sot_data) == InputFormat.SOT
    assert CompatibilityLayer.detect_input_format(json.dumps(request).encode()) == InputFormat.JSON
    logger.info("Format detection working correctly")
    
    logger.info("Basic test completed successfully!")

def run_client_test(model_path: str):
    """Run client integration test"""
    logger.info(f"Running client integration test with model: {model_path}")
    
    if not os.path.exists(model_path):
        logger.error(f"Model file not found: {model_path}")
        return
    
    test_audio = create_test_audio_data()
    
    try:
        with WhisperClient() as client:
            # Start server
            client.start_server(model_path)
            
            # Test SOT protocol
            logger.info("Testing SOT protocol...")
            sot_result = client.transcribe_with_sot(test_audio + SOTProcessor.SOT_MARKER)
            logger.info(f"SOT result: {sot_result[:100]}...")
            
            # Test JSON protocol
            logger.info("Testing JSON protocol...")
            json_options = TranscriptionOptions(
                language="en",
                include_timestamps=True,
                temperature=0.0
            )
            json_result = client.transcribe_with_json(test_audio, json_options)
            logger.info(f"JSON result: {json_result.text[:100]}...")
            logger.info(f"JSON language: {json_result.language}")
            logger.info(f"JSON success: {json_result.success}")
            
            logger.info("Client integration test completed successfully!")
            
    except Exception as e:
        logger.error(f"Client test failed: {str(e)}")

def main():
    """Main function with command line interface"""
    parser = argparse.ArgumentParser(
        description="Migration Example: SOT to JSON Interface Converter",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Convert SOT file to JSON
  python migration_example.py convert --input audio.sot --output audio.json
  
  # Convert JSON file to SOT-like format
  python migration_example.py convert --input audio.json --output audio.sot --format sot
  
  # Run basic functionality test
  python migration_example.py test --basic
  
  # Run client integration test
  python migration_example.py test --client --model model.bin
  
  # Convert from stdin to stdout
  cat audio.sot | python migration_example.py convert --format json
        """
    )
    
    subparsers = parser.add_subparsers(dest='command', help='Available commands')
    
    # Convert command
    convert_parser = subparsers.add_parser('convert', help='Convert between formats')
    convert_parser.add_argument('--input', '-i', help='Input file (stdin if not specified)')
    convert_parser.add_argument('--output', '-o', help='Output file (stdout if not specified)')
    convert_parser.add_argument('--format', '-f', choices=['json', 'sot'], 
                              default='json', help='Output format (default: json)')
    convert_parser.add_argument('--language', '-l', help='Language code for transcription')
    convert_parser.add_argument('--timestamps', '-t', action='store_true',
                              help='Include timestamps in output')
    convert_parser.add_argument('--temperature', type=float, default=0.0,
                              help='Temperature for sampling (0.0-1.0)')
    convert_parser.add_argument('--verbose', '-v', action='store_true',
                              help='Enable verbose output')
    
    # Test command
    test_parser = subparsers.add_parser('test', help='Run tests')
    test_subparsers = test_parser.add_subparsers(dest='test_type', help='Test type')
    
    basic_test_parser = test_subparsers.add_parser('basic', help='Run basic functionality test')
    client_test_parser = test_subparsers.add_parser('client', help='Run client integration test')
    client_test_parser.add_argument('--model', '-m', required=True,
                                  help='Path to Whisper model file')
    
    # Client command
    client_parser = subparsers.add_parser('client', help='Client examples')
    client_subparsers = client_parser.add_subparsers(dest='client_mode', help='Client mode')
    
    basic_client_parser = client_subparsers.add_parser('basic', help='Basic client example')
    advanced_client_parser = client_subparsers.add_parser('advanced', help='Advanced client example')
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        return
    
    if args.verbose:
        logging.getLogger().setLevel(logging.DEBUG)
    
    try:
        if args.command == 'convert':
            # Convert command
            options = None
            if args.language or args.timestamps or args.temperature != 0.0:
                options = TranscriptionOptions(
                    language=args.language,
                    include_timestamps=args.timestamps,
                    temperature=args.temperature
                )
            
            if args.input:
                # Convert file
                success = MigrationConverter.convert_file(
                    args.input, args.output, args.format, options
                )
                if not success:
                    sys.exit(1)
            else:
                # Convert from stdin to stdout
                success = MigrationConverter.convert_stdin_to_stdout(args.format)
                if not success:
                    sys.exit(1)
                    
        elif args.command == 'test':
            # Test command
            if args.test_type == 'basic':
                run_basic_test()
            elif args.test_type == 'client':
                run_client_test(args.model)
            else:
                test_parser.print_help()
                
        elif args.command == 'client':
            # Client examples
            if args.client_mode == 'basic':
                logger.info("Running basic client example...")
                # This would show basic client usage code
                print_basic_client_example()
            elif args.client_mode == 'advanced':
                logger.info("Running advanced client example...")
                # This would show advanced client usage code
                print_advanced_client_example()
            else:
                client_parser.print_help()
                
    except KeyboardInterrupt:
        logger.info("Operation cancelled by user")
        sys.exit(1)
    except Exception as e:
        logger.error(f"Error: {str(e)}")
        sys.exit(1)

def print_basic_client_example():
    """Print basic client usage example"""
    example_code = '''
# Basic Client Example
import migration_example

# Create test audio data
audio_data = migration_example.create_test_audio_data()

# Initialize client
with migration_example.WhisperClient() as client:
    # Start server
    client.start_server("model.bin")
    
    # Transcribe using JSON interface
    options = migration_example.TranscriptionOptions(
        language="en",
        include_timestamps=True,
        temperature=0.0
    )
    
    result = client.transcribe_with_json(audio_data, options)
    
    if result.success:
        print(f"Transcription: {result.text}")
        print(f"Language: {result.language}")
    else:
        print(f"Error: {result.error}")
'''
    print(example_code)

def print_advanced_client_example():
    """Print advanced client usage example"""
    example_code = '''
# Advanced Client Example
import migration_example
import json
import base64

class AdvancedWhisperClient:
    def __init__(self, server_path="./whisper-background-server"):
        self.server_path = server_path
        self.server_process = None
    
    def transcribe_file(self, audio_path, options=None):
        """Transcribe an audio file"""
        # Read and encode audio file
        with open(audio_path, 'rb') as f:
            audio_data = f.read()
        
        # Start server if not running
        if not self.server_process:
            self.start_server("model.bin")
        
        # Transcribe
        return self.transcribe_with_json(audio_data, options)
    
    def batch_transcribe(self, audio_paths, options=None):
        """Transcribe multiple files"""
        results = []
        for path in audio_paths:
            try:
                result = self.transcribe_file(path, options)
                results.append({"file": path, "result": result})
            except Exception as e:
                results.append({"file": path, "error": str(e)})
        return results
    
    def convert_sot_to_json(self, sot_file, json_file):
        """Convert SOT file to JSON format"""
        return migration_example.MigrationConverter.convert_file(
            sot_file, json_file, "json"
        )

# Usage example
if __name__ == "__main__":
    client = AdvancedWhisperClient()
    
    # Transcribe single file
    options = migration_example.TranscriptionOptions(
        language="auto",
        include_timestamps=True,
        temperature=0.2
    )
    
    result = client.transcribe_file("audio.wav", options)
    print(f"Transcribed: {result.text}")
    
    # Batch transcribe
    audio_files = ["audio1.wav", "audio2.wav", "audio3.wav"]
    batch_results = client.batch_transcribe(audio_files, options)
    
    for item in batch_results:
        if "error" in item:
            print(f"Error with {item['file']}: {item['error']}")
        else:
            print(f"{item['file']}: {item['result'].text}")
'''
    print(example_code)

if __name__ == "__main__":
    main()