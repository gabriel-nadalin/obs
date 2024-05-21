use num_traits::pow;
use rand;

use crate::io::wav_reader;
use crate::DUTY_MAX;
use crate::SAMPLE_RATE;
use crate::BUFFER_SIZE;

use super::voice::Voice;

// enum DrumVoice {
//     Kick(Vec<u8>),
//     Snare(Vec<u8>),
//     HiHat(Vec<u8>),
//     Cymbal(Vec<u8>),
// }

pub struct DrumVoice {
    voice: Voice,
    freq: u32,
    duty: f32,
    pos: u32,
    samples_per_beat: u32,
    beat: u32,
    decay: f32,
    env: f32,
}

impl DrumVoice {

    pub fn new(bpm: u32, freq: u32, duty: f32) -> Self {
        let samples_per_beat = SAMPLE_RATE * 60 / bpm;
        let voice = Voice::new(freq, duty);
        let decay = 1. - pow(0.1, 7);
        Self {
            freq,
            duty,
            voice,
            samples_per_beat,
            decay,
            pos: 0,
            beat: 0,
            env: 1.,
        }
    }

    fn load_samples(&mut self) {
    }

    pub fn get_sample(&mut self) -> bool {
        if self.pos >= self.samples_per_beat {
            self.beat = (self.beat + 1) % 16;
            self.env = 1.;
            self.pos = 0;
            self.voice.set(self.freq, self.duty);
        }

        self.env *= self.decay;

        if self.pos % (SAMPLE_RATE / 1000) == 0 {
            let freq = (self.voice.freq() as f32 * self.env) as u32;
            let duty = self.voice.duty() * self.env;
            self.voice.set(freq, duty);
        }

        
        let mut sample = self.voice.out();

        let noise = 0.9;

        sample = sample & (rand::random::<f32>() < noise);

        self.pos += 1;

        return sample;
    }
}