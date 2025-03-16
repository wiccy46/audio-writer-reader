use cpal::traits::DeviceTrait;

#[derive(Clone)]
pub struct AudioEngineSettings {
    pub input_device: cpal::Device,
    pub input_config: cpal::StreamConfig,
    pub output_device: cpal::Device,
    pub output_config: cpal::StreamConfig,
    pub num_input_channels: u16,
    pub num_output_channels: u16,
    pub sample_rate: u32,
    pub buffer_size: u32,
}

impl AudioEngineSettings {
    pub fn new(
        input_device: cpal::Device, 
        output_device: cpal::Device,
        input_channels: u16,
        output_channels: u16,
    ) -> Self {
        let mut input_config = input_device.default_input_config().unwrap().config();
        let mut output_config = output_device.default_output_config().unwrap().config();

        let num_input_channels = input_config.channels as u16;
        let num_output_channels = output_config.channels as u16;

        if input_channels > 0 && input_channels <= num_input_channels {
            input_config.channels = input_channels;
        } else {
            log::warn!("Invalid input channels {}. Using maximum available: {}", input_channels, num_input_channels);
            input_config.channels = num_input_channels;
        }

        if output_channels > 0 && output_channels <= num_output_channels {
            output_config.channels = output_channels;
        } else {
            log::warn!("Invalid output channels {}. Using maximum available: {}", output_channels, num_output_channels);
            output_config.channels = num_output_channels;
        }
        
        Self {
            input_device,
            input_config: input_config.clone(),
            output_device,
            output_config: output_config.clone(),
            num_input_channels,
            num_output_channels,
            sample_rate: input_config.sample_rate.0,
            buffer_size: 1024, 
        }
    }

    pub fn update_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
        self.input_config.sample_rate = cpal::SampleRate(sample_rate);
        self.output_config.sample_rate = cpal::SampleRate(sample_rate);
    }

    pub fn update_buffer_size(&mut self, buffer_size: u32) {
        self.buffer_size = buffer_size;
        self.input_config.buffer_size = cpal::BufferSize::Fixed(buffer_size);
        self.output_config.buffer_size = cpal::BufferSize::Fixed(buffer_size);
    }

    pub fn get_config_for_device(&self, is_input: bool) -> cpal::StreamConfig {
        if is_input {
            self.input_config.clone()
        } else {
            self.output_config.clone()
        }
    }

    pub fn get_supported_input_sample_rates(&self) -> Vec<u32> {
        let mut supported_sample_rates = Vec::new();
        for range in self.input_device.supported_input_configs().unwrap() {
            supported_sample_rates.push(range.min_sample_rate().0);
            supported_sample_rates.push(range.max_sample_rate().0);
        }
        supported_sample_rates
    }

    pub fn get_supported_output_sample_rates(&self) -> Vec<u32> {
        let mut supported_sample_rates = Vec::new();
        for range in self.output_device.supported_output_configs().unwrap() {
            supported_sample_rates.push(range.min_sample_rate().0);
            supported_sample_rates.push(range.max_sample_rate().0);
        }
        supported_sample_rates
    }

    /// This method assumes all ranges have the same min and max buffer size
    pub fn get_supported_input_buffer_sizes(&self) -> (u32, u32) {
        for range in self.input_device.supported_input_configs().unwrap() {
            match range.buffer_size() {
                cpal::SupportedBufferSize::Range { min, max } => return (*min, *max),
                cpal::SupportedBufferSize::Unknown => return (0, 0),
            }
        }
        (0, 0)
    }

    pub fn get_supported_output_buffer_sizes(&self) -> (u32, u32) {
        for range in self.output_device.supported_output_configs().unwrap() {
            match range.buffer_size() {
                cpal::SupportedBufferSize::Range { min, max } => return (*min, *max),
                cpal::SupportedBufferSize::Unknown => return (0, 0),
            }
        }
        (0, 0)
    }
}