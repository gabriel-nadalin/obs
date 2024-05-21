use alsa::Direction;
use alsa::pcm::PCM;
use midly::stream::Buffer;
use obs::synth::drum_machine::{DrumMachine, DrumVoice};
use obs::io::player::Player;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufWriter, Write};
use wav;

use obs::synth::{drum_machine};
use obs::{utils, io::midi_reader};
use obs::io::wav_reader;
use obs::SAMPLE_RATE;
use obs::AMPLITUDE_MAX;

fn main() {
    let pcm = PCM::new("default", Direction::Playback, false).unwrap();

    utils::set_pcm_params(&pcm);

    let mut reader = midi_reader::MidiFile::new(include_bytes!("../../musicas/ice_cap.mid"));
    reader.list_tracks();
    
    // In case the buffer was larger than 2 seconds, start the stream manually.
    // if pcm.state() != State::Running { pcm.start().unwrap() };
    // Wait for the stream to finish playback.
    pcm.drain().unwrap();
}
