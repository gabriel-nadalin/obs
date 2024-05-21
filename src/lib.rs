pub mod synth;
pub mod utils;
pub mod io;

pub const SAMPLE_RATE: u32 = 300_000;
pub const AMPLITUDE_MIN: u8 = 0;
pub const AMPLITUDE_MAX: u8 = 100;
pub const BUFFER_SIZE: usize = 2048;
pub const VOICES_MAX: usize = 4;
pub const CHANNELS_MAX: usize = 16;
pub const DUTY_MAX: u32 = 1_0_000;