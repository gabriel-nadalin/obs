pub mod audio;
pub mod utils;
pub mod io;

pub const SAMPLE_RATE: u32 = 300_000;
pub const AMPLITUDE: u8 = 100;
pub const BUFFER_SIZE: usize = 1;
pub const VOICES_MAX: usize = 4;
pub const CHANNELS_MAX: usize = 4;
pub const DUTY_MAX: u32 = 1_0_000;
