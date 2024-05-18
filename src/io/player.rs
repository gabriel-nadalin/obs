use std::{time::Duration, io, thread};
use console::Term;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{stdin, stdout, Write};
use crate::audio::voice::Voice;
use crate::io::audio_out::AudioOut;
use crate::{AMPLITUDE, SAMPLE_RATE};

pub struct Player {
    output: AudioOut,
}

impl Player {

    pub fn new() -> Self {
        let output = AudioOut::new();
        Self {
            output,
        }
    }

    // pub fn play_samples(&mut self, samples: Vec<u8>) {
    //     let io = self.pcm.io_u8().unwrap();
    //     io.writei(&samples).unwrap();
    //     self.pcm.drain().unwrap();
    // }

    pub fn audio_out(&mut self, sample: bool) {
        self.output.audio_out(sample);
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
                self.output.audio_out(voice.out());
            }

            if let Some(Ok(Event::Key(Key::Char(ch)))) = event {
                write!(stdout, "Key pressed: {}\r\n", ch).unwrap();
            }

            if key_pressed {
                self.output.audio_out(voice.out());
                // key_pressed = false;
            }

            stdout.flush().unwrap();
            // sleep(Duration::from_millis(50)); // Add a small delay to reduce CPU usage
        }
    }
}