# Audio Writer Reader

A Rust CLI application for playing an audio file to an audio input device and recording it from another audio output device.

## Features

- Play WAV audio files through specified output audio devices
- Record audio from input devices while playback is active
- Save the recorded audio to a new WAV file
- Configure input and output channels
- List available audio devices

## Requirements

- Rust (2021 edition or later)
- Audio devices with appropriate input/output capabilities

## Installation

Clone the repository and build the application:

```bash
git clone https://github.com/yourusername/audio-writer-reader.git
cd audio-writer-reader
cargo build --release
```

The compiled binary will be available at `target/release/audio-writer-reader`.

## Usage

```bash
# List all available audio devices (no other parameters required)
audio-writer-reader --list-devices

# Basic usage
audio-writer-reader --input-file input.wav --output-file output.wav

# Advanced usage with custom device names and channels
audio-writer-reader \
    --input-file input.wav \
    --output-file output.wav \
    --input-device "Device Name" \
    --output-device "Other Device" \
    --input-channels 4 \
    --output-channels 2
```

### Command-line Arguments

| Argument | Description | Default |
|----------|-------------|---------|
| `--input-file`, `-i` | Input audio file to play | (required unless using --list-devices) |
| `--output-file`, `-o` | Output file path for the recorded audio | (required unless using --list-devices) |
| `--input-device` | Input device name | "default" |
| `--output-device` | Output device name | "default" |
| `--input-channels` | Number of input channels to record | 2 |
| `--output-channels` | Number of output channels to play | 2 |
| `--list-devices` | List all available audio devices and exit | |
| `--help`, `-h` | Show help information | |

## Audio Device Names

This application uses the CPAL (Cross-Platform Audio Library) to interface with audio hardware. 
The device names reported by `--list-devices` will likely differ from what you see when using 
ALSA tools like `aplay -L` or `arecord -L` on Linux. 

When specifying a device name, you have several options:
1. Use the exact name as shown by `--list-devices`
2. Use a partial name (case insensitive matching)
3. For Shanling UA2 devices, you can use either "UA2" or "Shanling" as identifiers

Examples:
```bash
# Use the exact device name
audio-writer-reader --input-file input.wav --output-file output.wav \
    --input-device "pipewire" --output-device "default"

# Use partial name matching
audio-writer-reader --input-file input.wav --output-file output.wav \
    --input-device "pipe" --output-device "UA2"
```

If your specified name matches multiple devices, the application will list all matching devices
and ask you to be more specific.

Device name differences occur because:
1. CPAL abstracts over different audio backends (ALSA, PulseAudio, JACK)
2. It uses its own device enumeration system
3. On Linux, it may prioritize higher-level audio servers like PulseAudio or PipeWire

## Audio Format

The application currently supports WAV files with the following specifications:
- Sample rate: 48kHz
- Bit depth: 24-bit
- Channels: Variable (depending on the input file)

## How it Works

1. The application reads the input WAV file
2. It configures audio streams for playback (output) and recording (input)
3. It plays the audio file through the specified output device
4. Simultaneously, it records audio from the specified input device
5. After playback completes, it saves the recorded audio to the output file

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

This is a Rust rewrite of an original C++ application, using modern audio libraries:
- cpal for audio device interaction and streaming
- hound for WAV file handling 