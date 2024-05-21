use alsa::Direction;
use alsa::pcm::PCM;
use midly::stream::Buffer;
use obs::synth::drum_machine::{DrumMachine, DrumVoice};
use obs::io::player::{Player, PlayerKind};
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufWriter, Write};
use wav;

use obs::synth;
use obs::synth::{drum_machine};
use obs::{utils, io::midi_reader};
use obs::io::wav_reader;
use obs::SAMPLE_RATE;
use obs::AMPLITUDE_MAX;

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

    let buffer = wav_reader::get_sample(File::open(Path::new("samples/snare")).unwrap());
    // let mut drums = DrumVoice::new(138, 180, 0.45);
    let mut player = Player::new(PlayerKind::KeyboardPlayer);
    let mut drums = DrumMachine::new();
    drums.load_voice(DrumVoice::new(buffer));

    // write to wav
    // let header = wav::Header::new(1, 1, SAMPLE_RATE, 8);
    // let mut writer = BufWriter::new(File::create("output.wav").expect("Failed to create WAV file"));
    // let mut buffer = vec![];
    // for _ in 0..SAMPLE_RATE * 2 {
    //     if drums.get_sample() {
    //         buffer.push(AMPLITUDE)
    //     } else {
    //         buffer.push(0)
    //     }
    // }
    // let track = wav::BitDepth::Eight(buffer);
    // wav::write(header, &track, &mut writer).unwrap();

    // play
    // loop {
    //     player.audio_out(drums.get_sample())
    // }
    
    // In case the buffer was larger than 2 seconds, start the stream manually.
    // if pcm.state() != State::Running { pcm.start().unwrap() };
    // Wait for the stream to finish playback.
    pcm.drain().unwrap();
}
