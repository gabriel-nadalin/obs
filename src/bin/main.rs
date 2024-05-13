use alsa::Direction;
use alsa::pcm::PCM;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;

use obs::audio::{synth};
use obs::{utils, io::midi_reader};
use obs::io::wav_reader;

fn main() {
    let pcm = PCM::new("default", Direction::Playback, false).unwrap();

    utils::set_pcm_params(&pcm);

    let io = pcm.io_u8().unwrap();

    let mut synth: synth::Synth = Default::default();

    let file1 = File::open("musicas/badapple_nomico_lead.txt").unwrap();
    let file2 = File::open("musicas/badapple_nomico_bass.txt").unwrap();
    let file3 = File::open("musicas/badapple_nomico8.txt").unwrap();
    let file4 = File::open("musicas/badapple_nomico7.txt").unwrap();

    let mut readers = [BufReader::new(file1), BufReader::new(file2), BufReader::new(file3), BufReader::new(file4)];

    // synth.play_files(&mut readers, &io);

    // midi_reader::play();
    // let mut reader = midi_reader::MidiReader::new(include_bytes!("../../musicas/ice_cap.mid"));
    // reader.list_tracks();

    // let reader = wav_reader::test();
    wav_reader::play_wav_sample(File::open(Path::new("samples/kick")).unwrap());
    
    // In case the buffer was larger than 2 seconds, start the stream manually.
    // if pcm.state() != State::Running { pcm.start().unwrap() };
    // Wait for the stream to finish playback.
    pcm.drain().unwrap();
}
