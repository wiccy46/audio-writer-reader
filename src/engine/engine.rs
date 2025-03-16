use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Stream, StreamConfig,
    Host, HostId,
};
use std::error::Error;
use crate::engine::settings::AudioEngineSettings;
use crate::engine::track::Track;

pub struct Engine {
    host: Host,
    settings: AudioEngineSettings,
    tracks: Vec<Track>,
    input_stream: Option<Stream>,
    output_stream: Option<Stream>,
}

impl Engine {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let input_device = host.default_input_device()
            .expect("no input device available");
        let input_config = input_device.default_input_config()
            .expect("no input config available")
            .config();

        let output_device = host.default_output_device()
            .expect("no output device available");
        let output_config = output_device.default_output_config()
            .expect("no output config available")
            .config();

        let settings = AudioEngineSettings::new(
            input_device,
            input_config,
            output_device,
            output_config,
        );

        let tracks = Vec::new();

        Engine {
            host,
            settings,
            tracks,
            input_stream: None,
            output_stream: None,
        }
    }

    fn rebuild_streams(&mut self) -> Result<(), Box<dyn Error>> {
        // Stop existing streams if any
        self.input_stream = None;
        self.output_stream = None;

        // Configure new streams with current settings
        // Note: Actual stream building logic would go here
        // This is just a placeholder for the structure
        
        Ok(())
    }

    pub fn get_current_settings(&self) -> AudioEngineSettings {
        self.settings.clone()
    }

    pub fn set_host(&mut self, host: Option<HostId>) -> Result<(), Box<dyn Error>> {
        let new_host = if let Some(host_id) = host {
            cpal::host_from_id(host_id)?
        } else {
            cpal::default_host()
        };

        self.host = new_host;
        
        Ok(())
    }

    /// device.default_input_config() returns a StreamConfig, this is used for constructing a stream
    /// device.supported_input_configs() returns a SupportedInputConfigs that is an iterator of SupportedStreamConfigRange
    /// There it has channels, min_sample_rate, max_sample_rate, buffer_size, sample_format
    pub fn set_input_device(&mut self, device: &str) -> Result<(), Box<dyn Error>> {
        let input_device = if device == "default" {
            self.host.default_input_device()
                .ok_or("No default input device available")?
        } else {
            self.host.input_devices()?
                .find(|x| x.name().map(|y| y == device).unwrap_or(false))
                .ok_or("Input device not found")?
        };

        // Get supported configs
        let mut config = input_device.default_input_config()?.config();
        
        // Try to maintain current sample rate if supported
        let current_sample_rate = self.settings.sample_rate;
        let supports_sample_rate = input_device
            .supported_input_configs()?
            .any(|range| {
                range.min_sample_rate().0 <= current_sample_rate 
                && range.max_sample_rate().0 >= current_sample_rate
            });
            
        if supports_sample_rate {
            config.sample_rate = cpal::SampleRate(current_sample_rate);
        }

        // Update buffer size if fixed size is supported
        if let cpal::BufferSize::Fixed(_) = self.settings.input_config.buffer_size {
            config.buffer_size = cpal::BufferSize::Fixed(self.settings.buffer_size);
        }

        // Update settings in place
        self.settings.input_device = input_device;
        self.settings.input_config = config.clone();
        self.settings.num_input_channels = config.channels as usize;

        Ok(())
    }

    pub fn set_output_device(&mut self, device: &str) -> Result<(), Box<dyn Error>> {
        let output_device = if device == "default" {
            self.host.default_output_device()
                .ok_or("No default output device available")?
        } else {
            self.host.output_devices()?
                .find(|x| x.name().map(|y| y == device).unwrap_or(false))
                .ok_or("Output device not found")?
        };

        // Get supported configs
        let mut config = output_device.default_output_config()?.config();
        
        // Try to maintain current sample rate if supported
        let current_sample_rate = self.settings.sample_rate;
        let supports_sample_rate = output_device
            .supported_output_configs()?
            .any(|range| {
                range.min_sample_rate().0 <= current_sample_rate 
                && range.max_sample_rate().0 >= current_sample_rate
            });
            
        if supports_sample_rate {
            config.sample_rate = cpal::SampleRate(current_sample_rate);
        }

        // Update buffer size if fixed size is supported
        if let cpal::BufferSize::Fixed(_) = self.settings.output_config.buffer_size {
            config.buffer_size = cpal::BufferSize::Fixed(self.settings.buffer_size);
        }

        // Update settings in place
        self.settings.output_device = output_device;
        self.settings.output_config = config.clone();
        self.settings.num_output_channels = config.channels as usize;

        Ok(())
    }

    pub fn print_available_hosts(&self) {
        for host in cpal::platform::available_hosts() {
            println!("Available host: {:?}", host);
        }
    }

    pub fn print_available_input_devices(&self) {
        println!("Available input devices:");
        for device in self.host.input_devices().unwrap() {
            println!("  {:?}", device.name().unwrap());
            if let Ok(supported_configs) = device.supported_input_configs() {
                println!("    Supported configurations:");
                for range in supported_configs {
                    println!("      Channels: {}", range.channels());
                    println!("      Sample rate range: {} - {} Hz",
                        range.min_sample_rate().0,
                        range.max_sample_rate().0);
                    println!("      Buffer size: {:?}", range.buffer_size());
                    println!("      Sample format: {:?}", range.sample_format());
                }
            }
        }
    }

    pub fn print_available_output_devices(&self) {
        println!("Available output devices:");
        for device in self.host.output_devices().unwrap() {
            println!("  {:?}", device.name().unwrap());
            if let Ok(supported_configs) = device.supported_output_configs() {
                println!("    Supported configurations:");
                for range in supported_configs {
                    println!("      Channels: {}", range.channels());
                    println!("      Sample rate range: {} - {} Hz",
                        range.min_sample_rate().0,
                        range.max_sample_rate().0);
                    println!("      Buffer size: {:?}", range.buffer_size());
                    println!("      Sample format: {:?}", range.sample_format());
                }
            }
        }
    }

    pub fn print_supported_buffer_sizes(&self) {
        let (min, max) = self.settings.get_supported_input_buffer_sizes();
        println!("Supported input buffer sizes: {} - {}", min, max);
        let (min, max) = self.settings.get_supported_output_buffer_sizes();
        println!("Supported output buffer sizes: {} - {}", min, max);
    }

    pub fn list_devices(&self) {
        self.print_available_input_devices();
        self.print_available_output_devices();
        self.print_supported_buffer_sizes();
    }
}