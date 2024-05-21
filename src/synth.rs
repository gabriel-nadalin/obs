pub mod voice;
pub mod channel;
pub mod drum_machine;
pub mod sampler;

use std::fs::File;
use std::io::BufReader;
use alsa::pcm::IO;
use num_traits::pow;

use crate::synth::channel::Channel;
use crate::utils::{read_message, write_buffer};
use crate::{AMPLITUDE_MAX, AMPLITUDE_MIN, CHANNELS_MAX};
use crate::SAMPLE_RATE;
use crate::BUFFER_SIZE;

fn start_channels() -> [Channel; CHANNELS_MAX] {
    let mut channels = vec![];
    for _ in 0..CHANNELS_MAX {
        channels.push(Channel::new());
    }
    let channels: [Channel; CHANNELS_MAX] = channels.try_into().unwrap();
    channels
}

/// combines the output of up to `CHANNELS_MAX` channels using PIM (Pulse Interleaving Method)
/// 
/// like multiple instruments playing different parts together
#[derive(Debug, Default)]
pub struct Synth {
    channels: [Channel; CHANNELS_MAX],
    current: usize,
    selected: usize
}

impl Synth {

    pub fn new() -> Self {
        let channels = start_channels();
        Self {
            channels,
            current: 0,
            selected: 0,
        }
    }

    /// returns `BUFFER_SIZE` next samples as u8
    pub fn get_buffer(&mut self) -> [u8; BUFFER_SIZE] {
        let mut buffer = [0; BUFFER_SIZE];
        for i in 0..BUFFER_SIZE {
            buffer[i] = self.get_sample();
        }
        buffer
    }

    /// returns `BUFFER_SIZE` next samples as bool
    pub fn get_buffer_bool(&mut self) -> [bool; BUFFER_SIZE] {
        let mut buffer = [false; BUFFER_SIZE];
        for i in 0..BUFFER_SIZE {
            buffer[i] = self.get_sample_bool();
        }
        buffer
    }

    /// returns next sample as u8
    pub fn get_sample(&mut self) -> u8 {
        let mut out = false;

        for i in 0..CHANNELS_MAX {
            let channel_out = self.channels[i].out();
            if i == self.current {
                out = channel_out;
            }
        }
        self.current = (self.current + 1) % CHANNELS_MAX;
        if out {AMPLITUDE_MAX} else {AMPLITUDE_MIN}
    }

    /// returns next sample as bool
    pub fn get_sample_bool(&mut self) -> bool {
        let mut out = false;

        for i in 0..CHANNELS_MAX {
            let channel_out = self.channels[i].out();
            if i == self.current {
                out = channel_out;
            }
        }
        self.current = (self.current + 1) % CHANNELS_MAX;
        out
    }

    /// select channel at index `i`
    pub fn channel_select(&mut self, i: usize) {
        if i < CHANNELS_MAX {
            self.selected = i;
        }
        else {
            println!("Invalid channel!");
        }
    }

    /// turns on note in selected channel
    pub fn note_on(&mut self, freq: u32, duty: f32, channel_n: usize) {
        self.channels[channel_n].note_on(freq, duty);
    }

    /// turn off note in selected channel
    pub fn note_off(&mut self, freq: u32, channel_n: usize) {
        self.channels[channel_n].note_off(freq);
    }

    // / TODO replace this with something better
    // pub fn play_files(&mut self, readers: &mut [BufReader<File>; CHANNELS_MAX], io: &IO<u8>) {
    //     for i in 0..CHANNELS_MAX {
    //         let parts = read_message(&mut readers[i]);
    //         self.channels[i].message.0 = parts[0];
    //         self.channels[i].message.1 = parts[1];
    //         let delay = parts[2];
    //         self.channels[i].message.2 = (delay as u64 * (SAMPLE_RATE as u64 / BUFFER_SIZE as u64) / pow(10, 6) as u64) as u32;
    //     }

    //     loop {
    //         for i in 0..CHANNELS_MAX {
    //             let (status, freq, counter) = self.channels[i].message;
    //             if counter == 0 {
    //                 // println!("{:?}", freqs);
    //                 if status == 0 {
    //                     self.channels[i].note_off(freq);
    //                 } else {
    //                     self.channels[i].note_on(freq, 0.1)
    //                 }
    
    //                 let parts = read_message(&mut readers[i]);
    
    //                 if parts.len() == 0 {
    //                     break;
    //                 }
    
    //                 self.channels[i].message.0 = parts[0];
    //                 self.channels[i].message.1 = parts[1];
    //                 let delay = parts[2];
    //                 self.channels[i].message.2 = (delay as u64 * (SAMPLE_RATE as u64 / BUFFER_SIZE as u64) / pow(10, 6) as u64) as u32;
    
    //                 // println!("{}, {}, {}", status, freq, delay);
    //             } else {
    //                 self.channels[i].message.2 -= 1;
    
    //             }

    //         }
    //         write_buffer(self.synth_out_buffer(), io);
    //     }
    // }
}