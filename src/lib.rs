pub mod audio;
pub mod utils;
pub mod io;

const SAMPLE_RATE: u32 = 300_000;
const AMPLITUDE: u8 = 100;
const BUFFER_SIZE: usize = 1;
const VOICES_MAX: usize = 4;
const CHANNELS_MAX: usize = 4;
const DUTY_MAX: u32 = 1000;
