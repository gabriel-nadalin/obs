use midly::{Smf};
use num_traits::pow;

use crate::synth::channel::Channel;

#[derive(Clone, Copy, Debug)]
enum MidiEventKind {
    NoteOff,
    NoteOn,
    MetaSetTempo(u32),
    Other,
}

#[derive(Clone, Copy)]
pub struct MidiEvent {
    kind: MidiEventKind,
    key: u8,
    velocity: u8,
    delta_tick: u32,
    channel: u8,
}

impl MidiEvent {
    pub fn kind(self) -> MidiEventKind {
        self.kind
    }

    pub fn key(self) -> u8 {
        self.key
    }

    pub fn velocity(self) -> u8 {
        self.velocity
    }

    pub fn delta_tick(self) -> u32 {
        self.delta_tick
    }

    pub fn channel(self) -> u8 {
        self.channel
    }

    
}

pub struct MidiNote {
    key: u8,
    velocity: u8,
    start_time: u8,
    duration: u8,
}

pub struct MidiTrack {
    name: String,
    instrument: String,
    events: Vec<MidiEvent>,
    notes: Vec<MidiNote>,
    cursor: usize,
}

impl MidiTrack {
    pub fn get_event(&self, index: usize) -> &MidiEvent {
        &self.events[index]
    }
}

pub struct MidiFile {
    tracks: Vec<MidiTrack>,
    tempo: u32,
    ticks_per_beat: u16,
}

impl MidiFile {
    pub fn new(f: &[u8]) -> Self {
        let smf = Smf::parse(f).unwrap();
        let tempo = 500_000;        // default midi tempo
        let tracks = Self::parse_tracks(&smf);
        let ticks_per_beat = match smf.header.timing {
            midly::Timing::Metrical(value) => value.as_int(),
            midly::Timing::Timecode(_, _) => {
                // Handle Timecode variant if needed
                // For now, just use a default value
                0
            }
        };
        Self {
            tracks,
            tempo,
            ticks_per_beat,
        }
    }

    pub fn get_next_event(&mut self, track_n: usize) -> MidiEvent {
        let track = &mut self.tracks[track_n];
        let cursor = &mut track.cursor;
        let event = track.events[*cursor];
        *cursor += 1;
        event
    }

    fn parse_tracks(smf: &Smf) -> Vec<MidiTrack> {
        let mut tracks = vec![];

        for (i, track_midly) in smf.tracks.iter().enumerate() {
            let mut name = String::from("");
            let mut instrument = String::from("");
            let mut events = vec![];
            let mut notes = vec![];
            let cursor = 0;

            // parsing and storing track events
            for event in track_midly {

                match event.kind {

                    // check if the event is a meta message
                    midly::TrackEventKind::Meta(info) => {
                        match info {
                            midly::MetaMessage::TrackName(track_name) => {
                                for char in track_name {
                                    name.push(*char as char);
                                }
                                events.push(MidiEvent {
                                    kind: MidiEventKind::Other,
                                    key: 0,
                                    velocity: 0,
                                    delta_tick: 0,
                                    channel: 0,
                                })
                            }
                            midly::MetaMessage::InstrumentName(inst_name) => {
                                for char in inst_name {
                                    instrument.push(*char as char);
                                }
                                events.push(MidiEvent {
                                    kind: MidiEventKind::Other,
                                    key: 0,
                                    velocity: 0,
                                    delta_tick: 0,
                                    channel: 0,
                                })
                            }
                            midly::MetaMessage::Tempo(tempo) => {
                                events.push(MidiEvent {
                                    kind: MidiEventKind::MetaSetTempo(tempo.as_int()),
                                    key: 0,
                                    velocity: 0,
                                    delta_tick: 0,
                                    channel: 0,
                                })
                            }
                            _ => {
                                events.push(MidiEvent {
                                    kind: MidiEventKind::Other,
                                    key: 0,
                                    velocity: 0,
                                    delta_tick: 0,
                                    channel: 0,
                                })
                            }
                        }
                    }

                    // else check if event is of interest (note on or off for now)
                    midly::TrackEventKind::Midi { channel, message } => {
                        match message {
                            midly::MidiMessage::NoteOn { key, vel } => {
                                let mut kind = MidiEventKind::NoteOn;
    
                                // by convention, a NoteOn message with 0 velocity should be treated as a NoteOff
                                if vel == 0 {
                                    kind =MidiEventKind::NoteOff;
                                }
    
                                events.push(MidiEvent {
                                    kind,
                                    key: key.as_int(),
                                    velocity: vel.as_int(),
                                    delta_tick: event.delta.as_int(),
                                    channel: channel.as_int(),
                                });
                            }
                            midly::MidiMessage::NoteOff { key, vel } => {
                                events.push(MidiEvent {
                                    kind: MidiEventKind::NoteOff,
                                    key: key.as_int(),
                                    velocity: vel.as_int(),
                                    delta_tick: event.delta.as_int(),
                                    channel: channel.as_int(),
                                })
                            }
                            _ => {
                                events.push(MidiEvent {
                                    kind: MidiEventKind::Other,
                                    key: 0,
                                    velocity: 0,
                                    delta_tick: event.delta.as_int(),
                                    channel: channel.as_int(),
                                })
                            }
                        }
                    }

                    _ => {
                        events.push(MidiEvent {
                            kind: MidiEventKind::Other,
                            key: 0,
                            velocity: 0,
                            delta_tick: event.delta.as_int(),
                            channel: 0,
                        })
                    }
                }
            }

            tracks.push(MidiTrack {
                name,
                instrument,
                events,
                notes,
                cursor,
            })
        }
        tracks
    }

    fn delta2us(&self, delta_ticks: u32) -> u32 {
        self.tempo * delta_ticks / self.ticks_per_beat as u32
    }

    fn midi2freq(note: u8) -> u32 {
        pow(2, (note as usize - 69) / 12) * 440
    }

    pub fn tracks(&self) -> &Vec<MidiTrack> {
        &self.tracks
    }
    
    pub fn list_tracks(&mut self) {
        for (i, track) in self.tracks.iter().enumerate() {
            let name = &track.name;
            let n_messages = track.events.len();
            println!("{i} - track '{name}': {n_messages} messages");
        }
    }

    pub fn list_events(&mut self, track: usize) {
        for (i, event) in self.tracks[track].events.iter().enumerate() {
            println!("{:?}", event.kind);
            // let name = &track.name;
            // let n_messages = track.events.len();
            // println!("{i} - track '{name}': {n_messages} messages");
        }
    }

    // pub fn set_channels(&mut self) {
    //     for track in self.smf.tracks.iter() {
    //         for event in track {
    //             if let TrackEventKind::Midi { channel: channel_number, message } = event.kind {
    //                 let channel_number = MidiChannel {channel: Channel::default(), channel_number: channel_number.as_int() as usize};
    //                 self.channels.push(channel_number);
    //                 break;
    //             }
    //         }
    //     } 
    // }

    // fn next_message(&mut self, track: usize) -> TrackEvent {
    //     self.smf.tracks[track].pop().unwrap()
    // }
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
    // for track in smf.tracks {
    //     for event in track {
    //         // Check if the event is a MIDI message
    //         if let TrackEventKind::Midi { channel: _, message } = event.kind {
    //             // Process the MIDI message
    //             match message {
    //                 // MidiMessage::NoteOn { key, vel } => {
    //                 //     println!("Note On: key={}, velocity={}", key, vel);
    //                 // }
    //                 // MidiMessage::NoteOff { key, vel } => {
    //                 //     println!("Note Off: key={}, velocity={}", key, vel);
    //                 // }
    //                 // Handle other MIDI message types as needed
    //                 _ => {}
    //             }
    //         } else if let TrackEventKind::Meta(info) = event.kind {
    //             dbg!(info);
    //         }
    //     }
    // }

    // dbg!(&smf.tracks[3][7].message());
    // for (i, track) in smf.tracks.iter().enumerate() {
    //     println!("track {} has {} events", i, track.len());
    // }
}