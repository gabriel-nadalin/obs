use crate::DUTY_MAX;
use crate::SAMPLE_RATE;
use crate::BUFFER_SIZE;

/// generates a 1 bit square wave with frequency `freq`
#[derive(Debug, Default)]
pub struct Voice {
    freq: u32,          // wave's frequency in Hz (musical note played)
    duty: f32,          // wave's duty cycle 
    counter: u32,       // counts samples per wave period
    period: u32,        // wave's period in number of samples
    waveform: u32,      // duty cycle in number of samples
    on: bool            // self-explanatory
}

impl Voice {

    pub fn new(freq: u32, duty: f32) -> Self {
        let freq = freq;
        let duty = duty;
        let counter = 0;
        let period = SAMPLE_RATE / freq;
        let waveform = (period as f32 * duty) as u32;
        let on = true;
        Self {
            freq,
            duty,
            counter,
            period,
            waveform,
            on,
        }

    }

    pub fn freq(&mut self) -> u32 {
        self.freq
    }

    pub fn duty(&mut self) -> f32 {
        self.duty
    }

    /// sets voice's `freq` and `duty` and turns it on
    pub fn set(&mut self, freq: u32, duty: f32) {
        if freq == 0 {
            self.freq = 0;
            self.period = 0;
            self.on = false;
        } else {
            self.freq = freq;
            self.duty = duty;
            // self.counter = 0;
            self.period = SAMPLE_RATE / freq;
            self.waveform = (self.period as f32 * duty) as u32;
            self.on = true;
        }
    }

    /// turns voice off
    pub fn unset(&mut self) {
        self.on = false;
    }

    /// returns true if a voice is turned on and false otherwise
    pub fn is_on(&mut self) -> bool {
        self.on
    }

    /// returns `BUFFER_SIZE` next samples
    pub fn out_buffer(&mut self) -> [bool; BUFFER_SIZE] {
        let mut buffer = [false; BUFFER_SIZE];

        for i in 0..BUFFER_SIZE {
            self.counter += 1;
            if self.counter >= self.period {
                self.counter = 0;
            } else if self.counter < self.waveform {
                buffer[i] = true;
            } else {
                buffer[i] = false;
            }
        }

        buffer
    }

    /// returns next sample
    pub fn out(&mut self) -> bool {
        self.counter += 1;
        if self.counter >= self.period {
            self.counter = 0;
            false
        } else if self.counter < self.waveform {
            true
        } else {
            false
        }
    }
}