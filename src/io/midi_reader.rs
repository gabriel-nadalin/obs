use midly::{MidiMessage, Smf, TrackEvent, TrackEventKind, Timing, MetaMessage};
use num_traits::pow;

use crate::synth::channel::Channel;

enum MidiEventType {
    NoteOff,
    NoteOn,
    Other,
}

struct MidiEvent {
    r#type: MidiEventType,
    key: u8,
    velocity: u8,
    wall_tick: u32,
    delta_tick: u32,
}

struct MidiNote {
    key: u8,
    velocity: u8,
    start_time: u8,
    duration: u8,
}

struct MidiTrack <'a>{
    name: String,
    instrument: String,
    events: Vec<TrackEvent<'a>>,
    notes: Vec<MidiNote>,
    max_note: u8,
    min_note: u8,
}
struct MidiChannel {
    channel: Channel,
    channel_number: usize,
}

pub struct MidiReader<'a> {
    smf: Smf<'a>,
    tempo: u32,
    ticks_per_beat: u16,
    channels: Vec<MidiChannel>,
}

impl<'a> MidiReader<'a> {
    pub fn new(f: &'a [u8]) -> Self {
        let smf = Smf::parse(f).unwrap();
        let tempo = 500_000;        // default midi tempo
        let channels = vec![];
        let ticks_per_beat = match smf.header.timing {
            Timing::Metrical(value) => value.as_int(),
            Timing::Timecode(_, _) => {
                // Handle Timecode variant if needed
                // For now, just use a default value
                0
            }
        };
        Self {
            smf,
            tempo,
            ticks_per_beat,
            channels,
        }
    }

    fn delta2us(self, delta_ticks: u32) -> u32 {
        self.tempo * delta_ticks / self.ticks_per_beat as u32
    }

    fn midi2freq(note: u8) -> u32 {
        pow(2, (note as usize - 69) / 12) * 440
    }
    
    pub fn list_tracks(&mut self) {
        for (i, track) in self.smf.tracks.iter().enumerate() {
            let mut name = String::from("");
            let n_messages = track.len();
            for event in track {
                // Check if the event is a meta message
                if let TrackEventKind::Meta(info) = event.kind {
                    match info {
                        MetaMessage::TrackName(t_name) => {
                            for char in t_name {
                                name.push(*char as char);
                            }
                        }
                        _ => {}
                    }
                }
            }
            println!("{i} - track '{name}': {n_messages} messages");
        }
    }

    pub fn set_channels(&mut self) {
        for track in self.smf.tracks.iter() {
            for event in track {
                if let TrackEventKind::Midi { channel: channel_number, message } = event.kind {
                    let channel_number = MidiChannel {channel: Channel::default(), channel_number: channel_number.as_int() as usize};
                    self.channels.push(channel_number);
                    break;
                }
            }
        } 
    }

    fn next_message(&mut self, track: usize) -> TrackEvent {
        self.smf.tracks[track].pop().unwrap()
    }
}

pub fn play(){
    let smf = Smf::parse(include_bytes!("../../musicas/ice_cap.mid")).unwrap();

    // for track in smf.tracks {
    //     for event in track {
    //         if let TrackEventKind::Midi { channel, message } = event.kind {
                
    //         }
    //         dbg!(event.kind);
    //     }
    // }
    for track in smf.tracks {
        for event in track {
            // Check if the event is a MIDI message
            if let TrackEventKind::Midi { channel: _, message } = event.kind {
                // Process the MIDI message
                match message {
                    // MidiMessage::NoteOn { key, vel } => {
                    //     println!("Note On: key={}, velocity={}", key, vel);
                    // }
                    // MidiMessage::NoteOff { key, vel } => {
                    //     println!("Note Off: key={}, velocity={}", key, vel);
                    // }
                    // Handle other MIDI message types as needed
                    _ => {}
                }
            } else if let TrackEventKind::Meta(info) = event.kind {
                dbg!(info);
            }
        }
    }

    // dbg!(&smf.tracks[3][7].message());
    // for (i, track) in smf.tracks.iter().enumerate() {
    //     println!("track {} has {} events", i, track.len());
    // }
}