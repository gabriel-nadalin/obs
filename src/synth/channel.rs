use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use alsa::pcm::IO;
use num_traits::pow;
use rand::Rng;

use super::voice::Voice;
use crate::utils::{buffer_or, read_message, write_buffer};
use crate::{AMPLITUDE_MAX, AMPLITUDE_MIN, VOICES_MAX};
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

pub struct NoiseChannel {
    noise: f32,
}

impl NoiseChannel {

    pub fn new(noise: f32) -> Self {
        Self {
            noise,
        }
    }

    pub fn get_sample(&mut self) -> bool {
        rand::random::<f32>() < self.noise
    }


    pub fn generate_white_noise(size: usize) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        (0..size).map(|_| if rng.gen_bool(0.5) { 1.0 } else { -1.0 }).collect()
    }

    pub fn generate_pink_noise(size: usize) -> Vec<u8> {
        let white_noise = Self::generate_white_noise(size);
        let mut pink_noise = vec![0.0; size];
        let mut b = [0.02109238, 0.07113478, 0.68873558, -0.02813463, -0.02260048];
        let mut a = [1.0, -2.81337002, 2.69422456, -0.89651434, 0.02109238];
        
        for i in 0..size {
            let mut pink_sample = white_noise[i];
            for j in 1..b.len() {
                if i >= j {
                    pink_sample += b[j] * white_noise[i - j] - a[j] * pink_noise[i - j];
                }
            }
            pink_noise[i] = pink_sample;
        }

        pink_noise.into_iter().map(|sample| if sample >= 0.0 { AMPLITUDE_MAX } else { AMPLITUDE_MIN }).collect()
    }

    fn apply_low_pass_filter(input: &[f64], alpha: f64) -> Vec<f64> {
        let mut output = vec![0.0; input.len()];
        output[0] = input[0]; // Initialize the first sample
        for i in 1..input.len() {
            output[i] = alpha * input[i] + (1.0 - alpha) * output[i - 1];
        }
        output
    }

    pub fn generate_brown_noise(size: usize, alpha: f64) -> Vec<u8> {
        let white_noise = Self::generate_white_noise(size);
        let filtered_noise = Self::apply_low_pass_filter(&white_noise, alpha);
        
        // Convert to 1-bit
        filtered_noise.into_iter().map(|sample| if sample >= 0.0 { AMPLITUDE_MAX } else { AMPLITUDE_MIN }).collect()
    }
}


struct FIRBandPassFilter {
    coefficients: Vec<f64>,
    buffer: VecDeque<f64>,
}

impl FIRBandPassFilter {
    fn new(low_cut: f64, high_cut: f64, sample_rate: f64, filter_order: usize) -> Self {
        let coefficients = FIRBandPassFilter::calculate_coefficients(low_cut, high_cut, sample_rate, filter_order);
        let buffer = VecDeque::from(vec![0.0; filter_order]);
        FIRBandPassFilter { coefficients, buffer }
    }

    fn calculate_coefficients(low_cut: f64, high_cut: f64, sample_rate: f64, filter_order: usize) -> Vec<f64> {
        let sinc = |x: f64| if x == 0.0 { 1.0 } else { (x * std::f64::consts::PI).sin() / (x * std::f64::consts::PI) };
        
        let fc1 = low_cut / sample_rate;
        let fc2 = high_cut / sample_rate;
        let mut h = vec![0.0; filter_order];
        let m = filter_order as isize - 1;

        for i in 0..filter_order {
            if i as isize == m / 2 {
                h[i] = 2.0 * (fc2 - fc1);
            } else {
                let x = i as isize - m / 2;
                h[i] = sinc(2.0 * fc2 * x as f64) - sinc(2.0 * fc1 * x as f64);
            }
        }

        let norm: f64 = h.iter().sum();
        h.iter_mut().for_each(|x| *x /= norm);
        h
    }

    fn process_sample(&mut self, sample: f64) -> f64 {
        self.buffer.pop_front();
        self.buffer.push_back(sample);
        self.coefficients.iter().zip(self.buffer.iter().rev()).map(|(h, &x)| h * x).sum()
    }
}

fn binary_to_bipolar(bit: u8) -> f64 {
    2.0 * bit as f64 - 1.0
}

fn bipolar_to_binary(bit: f64) -> u8 {
    if bit >= 0.0 { 1 } else { 0 }
}