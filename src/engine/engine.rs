mod track;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Stream, StreamConfig,
    Host, HostId,
};
use std::error::Error;

pub struct Engine {
    host: Host,
    input_device: Option<Device>,
    output_device: Option<Device>,
    input_stream: Option<Stream>,
    output_stream: Option<Stream>,

}

impl Engine {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let input_device = host.default_input_device();
        let output_device = host.default_output_device();   

        Engine {
            host,
            input_device,
            output_device,
            input_stream: None,
            output_stream: None,
        }

    }

    pub fn set_host(&mut self, host: Option<HostId>) -> Result<(), Box<dyn Error>> {
        if let Some(host) = host {
            self.host = cpal::host_from_id(host)?;
        } else {
            self.host = cpal::default_host();
        }

        Ok(())
    }

    pub fn set_input_device(&mut self, device: &str) -> Result<(), Box<dyn Error>> {
        if device == "default" {
            self.input_device = self.host.default_input_device();
        } else {
            self.input_device = self.host.input_devices()?
                .find(|x| x.name().map(|y| y == device).unwrap_or(false));
        }
        Ok(())
    }
    
    pub fn set_output_device(&mut self, device: &str) -> Result<(), Box<dyn Error>> {
        if device == "default" {
            self.output_device = self.host.default_output_device();
        } else {
            self.output_device = self.host.output_devices()?
                .find(|x| x.name().map(|y| y == device).unwrap_or(false));
        }
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
                    println!("      Sample format: {:?}", range.sample_format());
                }
            }
        }
    }

    pub fn list_devices(&self) {
        self.print_available_input_devices();
        self.print_available_output_devices();
    }
}