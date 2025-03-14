mod engine;

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use cpal::traits::{DeviceTrait, HostTrait};
use log::{debug, error, info};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[cfg(target_os = "linux")]
use cpal::HostId;

// Define the audio systems available on Linux
#[cfg(target_os = "linux")]
#[derive(Parser, Debug, Clone, ValueEnum)]
enum AudioSystem {
    Alsa,
    Pulse,
    #[cfg(feature = "jack")]
    Jack,
}

// Define the CLI arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input audio file to play
    #[arg(short, long, required_unless_present = "list_devices")]
    input_file: Option<String>,

    /// Output file path for the recorded audio
    #[arg(short, long, required_unless_present = "list_devices")]
    output_file: Option<String>,

    /// Input device name
    #[arg(long, default_value = "default")]
    input_device: String,

    /// Output device name
    #[arg(long, default_value = "default")]
    output_device: String,

    /// Number of input channels to record
    #[arg(long, default_value_t = 2)]
    input_channels: u16,

    /// Number of output channels to play
    #[arg(long, default_value_t = 2)]
    output_channels: u16,
    
    /// List all available audio devices and exit
    #[arg(long)]
    list_devices: bool,

    /// Audio system to use (Linux only)
    #[cfg(target_os = "linux")]
    #[arg(long)]
    audio_system: Option<AudioSystem>,
}

fn main() -> Result<()> {
    // Initialize the logger
    env_logger::init();
    
    // Parse command line arguments
    let args = Args::parse();
    
    let engine = engine::Engine::new();
    engine.print_available_hosts();
    engine.list_devices();
    
    Ok(())
}
