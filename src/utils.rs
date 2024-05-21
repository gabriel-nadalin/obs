use alsa::ValueOr;
use alsa::pcm::{Access, Format, HwParams, IO};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::SAMPLE_RATE;
use crate::BUFFER_SIZE;
use crate::AMPLITUDE_MAX;

pub fn set_pcm_params(pcm: &alsa::PCM) {
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(1).unwrap();
    hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::U8).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
}


pub fn read_message (reader: &mut BufReader<File>) -> Vec<u32>{
    let mut message = String::new();
    let mut parts = Vec::new();
    
    match reader.read_line(&mut message) {
        Ok(_) => {
            for part in message.split_whitespace() {
                parts.push(part.parse::<u32>().unwrap());
            }
            parts
        }
        Err(_) => panic!(), // Handle any errors
    }

}

pub fn buffer_or(buffer1: [bool; BUFFER_SIZE], buffer2: [bool; BUFFER_SIZE]) -> [bool; BUFFER_SIZE] {
    let mut buffer = [false; BUFFER_SIZE];

    for i in 0..BUFFER_SIZE {
        buffer[i] = buffer1[i] | buffer2[i];
    }

    buffer
}

pub fn write_buffer(buffer: [bool; BUFFER_SIZE], io: &IO<u8>) {
    let mut buffer_out = [0; BUFFER_SIZE];

    for i in 0..BUFFER_SIZE {
        buffer_out[i] = buffer[i] as u8 * AMPLITUDE_MAX;
    }

    assert_eq!(io.writei(&buffer_out).unwrap(), BUFFER_SIZE);
} 