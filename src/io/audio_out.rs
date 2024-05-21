use alsa::Direction;
use alsa::pcm::{Access, Format, HwParams, IO, PCM};
use alsa::ValueOr;
use crate::{AMPLITUDE_MAX, AMPLITUDE_MIN, BUFFER_SIZE, SAMPLE_RATE};

pub fn set_pcm_params(pcm: &alsa::PCM) {
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(1).unwrap();
    hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::U8).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
}

pub struct AudioOut {
    pcm: PCM,
    buffer: Vec<u8>
}

impl AudioOut {

    pub fn new() -> Self {
        let pcm = PCM::new("default", Direction::Playback, false).unwrap();
        set_pcm_params(&pcm);
        let buffer = vec![];
        Self {
            pcm,
            buffer,
        }
    }

    pub fn audio_out(&mut self, sample: bool) {
        let sample = if sample {AMPLITUDE_MAX} else {AMPLITUDE_MIN};
        self.buffer.push(sample);
        if self.buffer.len() >= BUFFER_SIZE {
            let io = self.pcm.io_u8().unwrap();
            io.writei(&self.buffer).unwrap();
            self.buffer.clear();
        }
    }
}