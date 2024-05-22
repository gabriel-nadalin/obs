use std::fs::File;
use std::path::Path;
use wav::BitDepth;
use crate::io::player::{Player};
use crate::{AMPLITUDE_MAX, AMPLITUDE_MIN, SAMPLE_RATE};

pub fn test() {
    let mut inp_file = File::open(Path::new("samples/kick.wav")).unwrap();
    let (header, data) = wav::read(&mut inp_file).unwrap();
    dbg!(header);
    let mut buffer = vec![];
    for sample in data.try_into_eight().unwrap() {
        let mut bit = AMPLITUDE_MIN;
        if sample > 127 {
            bit = AMPLITUDE_MAX;
        }
        buffer.push(bit)
    }
    // dbg!(data.as_eight().unwrap().to_vec());
    // let mut player = Player::new(PlayerKind::KeyboardPlayer);
    // player.play_samples(buffer);
    // player.play_samples(data.as_sixteen().unwrap().to_vec());
}

pub fn donwsample(data: BitDepth) -> Vec<bool> {
    let mut vec = vec![];
    match data {
        BitDepth::Eight(samples) => {
            for sample in samples {
                let mut bit = false;
                if sample > 129 {
                    bit = true;
                }
                vec.push(bit);
            }
        }
        BitDepth::Sixteen(samples) => {
            for sample in samples {
                let mut bit = false;
                if sample > 50 {
                    bit = true;
                }
                vec.push(bit);
            }
        }
        _ => panic!()
    }
    vec
}

pub fn resample(samples: Vec<bool>, in_rate: u32, out_rate: u32) -> Vec<bool> {
    let mut out_buffer = vec![];
    for sample in samples {
        for _ in 0..out_rate/in_rate {
            out_buffer.push(sample);
        }
    }
    out_buffer
}

pub fn get_sample(mut f: File) -> Vec<bool> {
    let (header, data) = wav::read(&mut f).unwrap();
    let sampling_rate = header.sampling_rate;
    resample(donwsample(data), sampling_rate, SAMPLE_RATE)
}

pub fn play_wav_sample(mut f: File) {
    let (header, data) = wav::read(&mut f).unwrap();
    // println!("{:?}", data);
    let sampling_rate = header.sampling_rate;
    let buffer = resample(donwsample(data), sampling_rate, SAMPLE_RATE);
    // let mut player = Player::new(PlayerKind::KeyboardPlayer);
    for sample in buffer {
        // player.audio_out(sample);
    }
}