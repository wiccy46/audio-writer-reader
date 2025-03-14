use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Track {
    pub input_channel: Option<usize>,  // For now just allow a single input channel per track
    pub buffer: VecDeque<f32>, // For output stream
    pub rec_arm: bool,         // Whether the track is recording
    pub playback_pos: usize,   // Current playback position
}


impl Track {
    pub fn new(input_channel: Option<usize>) -> Self {
        Track {
            input_channel,
            buffer: VecDeque::new(),
            rec_arm: false,
            playback_pos: 0,
        }
    }
    
    pub fn record(&mut self, sample: f32) {
        if self.rec_arm {
            self.buffer.push_back(sample);
        }
    }
    

    // Returns the next sample from the buffer
    pub fn play(&mut self) -> f32{
        if self.playback_pos < self.buffer.len() {
            let sample = self.buffer[self.playback_pos];
            self.playback_pos += 1;
            sample
        } else {
            0.0
        }
    }
}