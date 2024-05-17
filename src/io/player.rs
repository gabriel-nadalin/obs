use alsa::Direction;
use alsa::pcm::{Access, Format, HwParams, IO, PCM};
use alsa::ValueOr;
use crate::{AMPLITUDE, SAMPLE_RATE};

pub fn set_pcm_params(pcm: &alsa::PCM) {
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(1).unwrap();
    hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::U8).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
}

pub struct Player {
    counters: u32,
    pcm: PCM,
}

impl Player {

    pub fn new() -> Self {
        let pcm = PCM::new("default", Direction::Playback, false).unwrap();
        set_pcm_params(&pcm);
        Self {
            counters: 0,
            pcm,
        }
    }

    pub fn play_samples(&mut self, samples: Vec<u8>) {
        let io = self.pcm.io_u8().unwrap();
        io.writei(&samples).unwrap();
        self.pcm.drain().unwrap();
    }

    pub fn audio_out(&mut self, sample: bool) {
        let io = self.pcm.io_u8().unwrap();
        let sample = [if sample {AMPLITUDE} else {0}];
        io.writei(&sample).unwrap();
    }
}