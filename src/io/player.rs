use std::{time::Duration, io, thread};
use console::Term;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{stdin, stdout, Write};
use crate::synth::voice::Voice;
use crate::io::audio_out::AudioOut;
use crate::synth::Synth;
use crate::{AMPLITUDE_MAX, SAMPLE_RATE};

use crate::io::midi_reader::{MidiFile, MidiEvent};

pub enum PlayerEvent {
    KeyPress(char),
    MidiMessage(MidiEvent),
    // Add other event types as needed
}

pub trait PlayerMode {
    fn audio_out(&mut self, sample: u8);
    fn update(&mut self);
    fn process_event(&mut self, event: PlayerEvent);
    // fn start(&mut self);
    // fn stop(&mut self);
}

pub struct KeyboardPlayer {
    output: AudioOut,
    synth: Synth,
}

impl KeyboardPlayer {

    pub fn new() -> Self {
        Self {
            output: AudioOut::new(),
            synth: Synth::new(),
        }
    }
}

impl PlayerMode for KeyboardPlayer {
    fn audio_out(&mut self, sample: u8) {
        self.output.audio_out(sample);
    }

    fn update(&mut self) {
        
    }

    fn process_event(&mut self, event: PlayerEvent) {
        
    }
}


pub struct MidiPlayer {
    output: AudioOut,
    synth: Synth,
    file: MidiFile,
    pointers: Vec<usize>,
}

impl MidiPlayer {

    pub fn new(file: MidiFile) -> Self {
        let pointers = vec![0; file.tracks().len()];
        Self {
            output: AudioOut::new(),
            synth: Synth::new(),
            file,
            pointers,
        }
    }
}

impl PlayerMode for MidiPlayer {
    fn audio_out(&mut self, sample: u8) {
        self.output.audio_out(sample);
    }

    fn update(&mut self) {
        let file = &mut self.file;
        for i in 0..file.tracks().len() {
            let event = file.get_next_event(i);
            while event.delta_tick() == 0 {
                self.process_event(PlayerEvent::MidiMessage(event));
            }
        }
    }

    fn process_event(&mut self, event: PlayerEvent) {

    }
}

pub enum Mode {
    Keyboard(KeyboardPlayer),
    Midi(MidiPlayer),
}

pub struct Player {
    mode: Box<dyn PlayerMode>,
}

impl Player {

    pub fn new(mode: Box<dyn PlayerMode>) -> Self {
        Self { mode }
    }

    // pub fn play_samples(&mut self, samples: Vec<u8>) {
    //     let io = self.pcm.io_u8().unwrap();
    //     io.writei(&samples).unwrap();
    //     self.pcm.drain().unwrap();
    // }

    pub fn audio_out(&mut self, sample: u8) {
        self.mode.audio_out(sample);
    }

    pub fn keyboard_player(&mut self) {
        let mut voice = Voice::new(440, 0.5);

        let mut stdin = async_stdin().events();
        let mut stdout = stdout().into_raw_mode().unwrap();
        let mut key_pressed = false;

        write!(stdout, "Press 'W' to play sound, 'Q' to exit...\r\n").unwrap();
        stdout.flush().unwrap();

        loop {
            let event = stdin.next();

            if let Some(Ok(Event::Key(Key::Char('q')))) = event {
                write!(stdout, "Exiting...\r\n").unwrap();
                stdout.flush().unwrap();
                break;
            }

            if let Some(Ok(Event::Key(Key::Char('w')))) = event {
                key_pressed = true;
                write!(stdout, "Key pressed: W\r\n").unwrap();
                // self.output.audio_out(voice.out());
            }

            if let Some(Ok(Event::Key(Key::Char(ch)))) = event {
                write!(stdout, "Key pressed: {}\r\n", ch).unwrap();
            }

            if key_pressed {
                // self.output.audio_out(voice.out());
                // key_pressed = false;
            }

            stdout.flush().unwrap();
            // sleep(Duration::from_millis(50)); // Add a small delay to reduce CPU usage
        }
    }

    pub fn update(&mut self) {
        
        //TODO: both midi and keyboard players should be updated from the main instead of containing their own loops
    }

    pub fn drain(&mut self) {
        // self.output.drain();
    }
}