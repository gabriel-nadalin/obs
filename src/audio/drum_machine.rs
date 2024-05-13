use std::fs::File;

use crate::io::wav_reader;
use crate::SAMPLE_RATE;
use crate::BUFFER_SIZE;

enum DrumVoice {
    Kick(Vec<u8>),
    Snare(Vec<u8>),
    HiHat(Vec<u8>),
    Cymbal(Vec<u8>),
}

pub struct DrumMachine {
    pos: u32
}

impl DrumMachine {

    pub fn new() {

    }

    fn load_samples(&mut self) {
    }

    pub fn get_sample(&mut self) -> bool {
        
    }
}