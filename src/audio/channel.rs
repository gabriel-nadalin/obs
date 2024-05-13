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
    voices: [Voice; VOICES_MAX],
    freqs: [u32; VOICES_MAX],
    on: bool,
    counter: u32,
    pub message: (u32, u32, u32)
}

impl Channel {

    /// sets an available voice's `freq` and `duty` and turns it on
    pub fn note_on(&mut self, freq: u32, duty: u32) {        
        for i in 0..VOICES_MAX {
            if self.freqs[i] == 0 {
                self.freqs[i] = freq;
                self.voices[i].set(freq, duty);
                return
            }
        }
        panic!()
    }

    /// turns off voice currently playing `freq`
    pub fn note_off(&mut self, freq: u32){
        for i in 0..VOICES_MAX {
            if self.freqs[i] == freq {
                self.freqs[i] = 0;
                self.voices[i].unset();
                return
            }
        }
    }

    /// returns `BUFFER_SIZE` next samples
    pub fn out_buffer(&mut self) -> [bool; BUFFER_SIZE] {
        let mut buffer = [false; BUFFER_SIZE];

        for i in 0..VOICES_MAX {
            buffer = if self.voices[i].is_on() {
                buffer_or(buffer, self.voices[i].out_buffer())
            } else {
                buffer
            };
        }

        buffer
    }

    /// returns next sample
    pub fn out(&mut self) -> bool {
        let mut out = false;
        for i in 0..VOICES_MAX {
            out = if self.voices[i].is_on() {
                out | self.voices[i].out()
            } else {
                out
            };
        }
        out
    }

    /// *deprecated* plays proprietary format file
    pub fn play_file(&mut self, file: File, io: &IO<u8>) {
        let mut reader = BufReader::new(file);
        let mut parts = read_message(&mut reader);
        let mut status = parts[0];
        let mut freq = parts[1];
        let mut delay = parts[2];
        self.counter = (delay as u64 * (SAMPLE_RATE as u64 / BUFFER_SIZE as u64) / pow(10, 6) as u64) as u32;
        // self.counter = (delay / pow(10, 6)) * (SAMPLE_RATE / BUFFER_SIZE as u32);
        println!("{:?}", self.counter);

        loop {
            if self.counter == 0 {
                println!("{:?}", self.freqs);
                if status == 0 {
                    self.note_off(freq);
                } else {
                    self.note_on(freq, 62)
                }

                parts = read_message(&mut reader);

                if parts.len() == 0 {
                    break;
                }

                status = parts[0];
                freq = parts[1];
                delay = parts[2];
                self.counter = (delay as u64 * (SAMPLE_RATE as u64 / BUFFER_SIZE as u64) / pow(10, 6) as u64) as u32;

                println!("{}, {}, {}", status, freq, delay);
            } else {
                self.counter -= 1;

            }

            write_buffer(self.out_buffer(), io);
        }
    }
}