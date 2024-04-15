use alsa::{Direction, ValueOr};
use alsa::pcm::{Access, Format, HwParams, IO, PCM};
use std::fs::File;
use std::io::{BufRead, BufReader};
use num_traits::pow;

mod voice;

const SAMPLE_RATE: u32 = 220_000;
const AMPLITUDE: u8 = 100;
const BUFFER_SIZE: usize = 1;
const N_VOICES: usize = 4;
const N_CHANNELS: usize = 4;
const DUTY_MAX: u32 = 1000;

#[derive(Debug, Default)]
struct Voice {
    freq: u32,
    duty: u32,
    counter: u32,
    period: u32,
    waveform: u32,
    on: bool
}

impl Voice {
    fn voice_set(&mut self, freq: u32, duty: u32) {
        self.freq = freq;
        self.duty = duty;
        self.counter = 0;
        self.period = SAMPLE_RATE / freq;
        self.waveform = self.period * duty / DUTY_MAX;
        self.on = true;
    }

    fn voice_unset(&mut self) {
        self.on = false;
    }

    fn voice_out_buffer(&mut self) -> [bool; BUFFER_SIZE] {
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

    fn voice_out(&mut self) -> bool {
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

#[derive(Debug, Default)]
struct Channel {
    voices: [Voice; N_VOICES],
    freqs: [u32; N_VOICES],
    counter: u32,
    message: (u32, u32, u32)
}

impl Channel {
    fn voice_on(&mut self, freq: u32, duty: u32) {        
        for i in 0..N_VOICES {
            if self.freqs[i] == 0 {
                self.freqs[i] = freq;
                self.voices[i].voice_set(freq, duty);
                return
            }
        }
        panic!()
    }

    fn voice_off(&mut self, freq: u32){
        for i in 0..N_VOICES {
            if self.freqs[i] == freq {
                self.freqs[i] = 0;
                self.voices[i].voice_unset();
                return
            }
        }
    }

    fn channel_out_buffer(&mut self) -> [bool; BUFFER_SIZE] {
        let mut buffer = [false; BUFFER_SIZE];

        for i in 0..N_VOICES {
            buffer = if self.voices[i].on {
                buffer_or(buffer, self.voices[i].voice_out_buffer())
            } else {
                buffer
            };
        }

        buffer
    }

    fn channel_out(&mut self) -> bool {
        let mut out = false;
        for i in 0..N_VOICES {
            out = if self.voices[i].on {
                out | self.voices[i].voice_out()
            } else {
                out
            };
        }
        out
    }

    fn play_file(&mut self, file: File, io: &IO<u8>) {
        let mut reader = BufReader::new(file);
        let mut parts = read_message(&mut reader);
        let mut status = parts[0];
        let mut freq = parts[1];
        let mut delay = parts[2];
        self.counter = (delay as u64 * (SAMPLE_RATE as u64 / BUFFER_SIZE as u64) / pow(10, 6) as u64) as u32;
        // self.counter = (delay / pow(10, 6)) * (SAMPLE_RATE / BUFFER_SIZE as u32);
        println!("{:?}", self.counter);

        loop {
            if self.counter == 0 {
                println!("{:?}", self.freqs);
                if status == 0 {
                    self.voice_off(freq);
                } else {
                    self.voice_on(freq, 62)
                }

                parts = read_message(&mut reader);

                if parts.len() == 0 {
                    break;
                }

                status = parts[0];
                freq = parts[1];
                delay = parts[2];
                self.counter = (delay as u64 * (SAMPLE_RATE as u64 / BUFFER_SIZE as u64) / pow(10, 6) as u64) as u32;

                println!("{}, {}, {}", status, freq, delay);
            } else {
                self.counter -= 1;

            }

            write_buffer(self.channel_out_buffer(), io);
        }
    }
}


#[derive(Debug, Default)]
struct Synth {
    channels: [Channel; N_CHANNELS],
    current: usize
}

impl Synth {
    fn synth_out_buffer(&mut self) -> [bool; BUFFER_SIZE] {
        let mut buffer = [false; BUFFER_SIZE];
        for i in 0..BUFFER_SIZE {
            buffer[i] = self.synth_out();
        }
        buffer
    }

    fn synth_out(&mut self) -> bool {
        let mut out = false;
        let mut channel_out;

        for i in 0..N_CHANNELS {
            channel_out = self.channels[i].channel_out();
            if i == self.current {
                out = channel_out;
            }
        }
        self.current = (self.current + 1) % N_CHANNELS;
        out
    }

    fn play_files(&mut self, readers: &mut [BufReader<File>; N_CHANNELS], io: &IO<u8>) {
        for i in 0..N_CHANNELS {
            let parts = read_message(&mut readers[i]);
            self.channels[i].message.0 = parts[0];
            self.channels[i].message.1 = parts[1];
            let delay = parts[2];
            self.channels[i].message.2 = (delay as u64 * (SAMPLE_RATE as u64 / BUFFER_SIZE as u64) / pow(10, 6) as u64) as u32;
        }

        loop {
            for i in 0..N_CHANNELS {
                let (status, freq, counter) = self.channels[i].message;
                if counter == 0 {
                    // println!("{:?}", freqs);
                    if status == 0 {
                        self.channels[i].voice_off(freq);
                    } else {
                        self.channels[i].voice_on(freq, 62)
                    }
    
                    let parts = read_message(&mut readers[i]);
    
                    if parts.len() == 0 {
                        break;
                    }
    
                    self.channels[i].message.0 = parts[0];
                    self.channels[i].message.1 = parts[1];
                    let delay = parts[2];
                    self.channels[i].message.2 = (delay as u64 * (SAMPLE_RATE as u64 / BUFFER_SIZE as u64) / pow(10, 6) as u64) as u32;
    
                    // println!("{}, {}, {}", status, freq, delay);
                } else {
                    self.channels[i].message.2 -= 1;
    
                }

            }
            write_buffer(self.synth_out_buffer(), io);
        }
    }
}

fn pcm_set_params(pcm: &alsa::PCM) {
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(1).unwrap();
    hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::U8).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
}


fn read_message (reader: &mut BufReader<File>) -> Vec<u32>{
    let mut message = String::new();
    let mut parts = Vec::new();
    
    match reader.read_line(&mut message) {
        Ok(_) => {
            for part in message.split_whitespace() {
                parts.push(part.parse::<u32>().unwrap());
            }
            parts
        }
        Err(_) => panic!(), // Handle any errors
    }

}

fn buffer_or(buffer1: [bool; BUFFER_SIZE], buffer2: [bool; BUFFER_SIZE]) -> [bool; BUFFER_SIZE] {
    let mut buffer = [false; BUFFER_SIZE];

    for i in 0..BUFFER_SIZE {
        buffer[i] = buffer1[i] | buffer2[i];
    }

    buffer
}

fn write_buffer(buffer: [bool; BUFFER_SIZE], io: &IO<u8>) {
    let mut buffer_out = [0; BUFFER_SIZE];

    for i in 0..BUFFER_SIZE {
        buffer_out[i] = buffer[i] as u8 * AMPLITUDE;
    }

    assert_eq!(io.writei(&buffer_out).unwrap(), BUFFER_SIZE);
} 

fn main() {
    let pcm = PCM::new("default", Direction::Playback, false).unwrap();

    pcm_set_params(&pcm);

    let io = pcm.io_u8().unwrap();

    let mut synth: Synth = Default::default();

    let file1 = File::open("musicas/badapple_nomico_lead.txt").unwrap();
    let file2 = File::open("musicas/badapple_nomico_bass.txt").unwrap();
    let file3 = File::open("musicas/badapple_nomico8.txt").unwrap();
    let file4 = File::open("musicas/badapple_nomico7.txt").unwrap();

    let mut readers = [BufReader::new(file1), BufReader::new(file2), BufReader::new(file3), BufReader::new(file4)];

    synth.play_files(&mut readers, &io);

    // synth.channels[0].voice_on(261, 500);
    // synth.channels[1].voice_on(329, 500);
    // synth.channels[2].voice_on(392, 500);
    // synth.channels[3].voice_on(660, 500);

    // let file = match File::open("musicas/megalovania_lead.txt") {
    //     Ok(file) => file,
    //     Err(err) => {
    //         println!("Error opening file: {}", err);
    //         return;
    //     }
    // };

    // channel.play_file(file, &io);

    // let reader = BufReader::new(file);

    // for _ in 0..3 * SAMPLE_RATE / BUFFER_SIZE as u32 {
    //     write_buffer(synth.synth_out(), &io);
    // }


    // In case the buffer was larger than 2 seconds, start the stream manually.
    // if pcm.state() != State::Running { pcm.start().unwrap() };
    // Wait for the stream to finish playback.
    pcm.drain().unwrap();
}
