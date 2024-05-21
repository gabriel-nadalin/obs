

pub struct DrumVoice {
    sample: Vec<bool>,
    counter: usize,
    on_hit: bool,
}

impl DrumVoice {
    pub fn new(sample: Vec<bool>) -> Self {
        Self {
            sample,
            counter: 0,
            on_hit: false,
        }
    }

    pub fn hit(&mut self) {
        self.counter = 0;
        self.on_hit = true;
    }

    pub fn get_sample(&mut self) -> bool {
        let mut out = false;
        if self.on_hit {
            out = self.sample[self.counter];
            self.counter += 1;
            if self.counter >= self.sample.len() {
                self.on_hit = false
            }
        }
        out
    }
}

pub struct DrumMachine {
    voices: Vec<DrumVoice>,
    current: usize,
}

impl DrumMachine {

    pub fn new() -> Self {
        Self {
            voices: vec![],
            current: 0,
        }
    }

    pub fn load_voice(&mut self, voice: DrumVoice) {
        self.voices.push(voice);
        self.voices[0].hit();
    }

    pub fn get_sample(&mut self) -> bool {
        let mut out = false;
        if self.voices.len() > 0 {
            for i in 0..self.voices.len() {
                let channel_out: bool = self.voices[i].get_sample();
                if i == self.current {
                    out = channel_out;
                }
            }
            self.current = (self.current + 1) % self.voices.len();
        }
        out
    }
}