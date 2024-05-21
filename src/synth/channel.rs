use std::fs::File;
use std::io::BufReader;
use alsa::pcm::IO;
use num_traits::pow;

use super::voice::Voice;
use crate::utils::{buffer_or, read_message, write_buffer};
use crate::VOICES_MAX;
use crate::SAMPLE_RATE;
use crate::BUFFER_SIZE;


/// combines the output of up to `VOICES_MAX` voices using PPM (Pin Pulse Method)
/// 
/// like an instrument playing multiple notes simultaneously
#[derive(Debug, Default)]
pub struct Channel {
    voices: Vec<Voice>,
}

impl Channel {

    pub fn new() -> Self {
        Self {
            voices: vec![],
        }
    }

    /// sets an available voice's `freq` and `duty` and turns it on
    pub fn note_on(&mut self, freq: u32, duty: f32) {
        if self.voices.len() < VOICES_MAX {
            self.voices.push(Voice::new(freq, duty))
        }
        panic!()
    }

    /// turns off voice currently playing `freq`
    pub fn note_off(&mut self, freq: u32){
        self.voices.retain(|voice| voice.freq() != freq);
    }

    /// returns `BUFFER_SIZE` next samples
    pub fn out_buffer(&mut self) -> [bool; BUFFER_SIZE] {
        let mut buffer = [false; BUFFER_SIZE];

        for i in 0..self.voices.len() {
            buffer = buffer_or(buffer, self.voices[i].out_buffer())
        }

        buffer
    }

    /// returns next sample
    pub fn out(&mut self) -> bool {
        let mut out = false;
        for i in 0..self.voices.len() {
            out |= self.voices[i].out()
        }
        out
    }
}