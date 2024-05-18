use obs::{audio::voice::Voice, io::{audio_out::AudioOut, player::Player}};

fn main() {
    let mut player = Player::new();
    player.keyboard_player();
    let mut output = AudioOut::new();
    let mut voice = Voice::new(440, 0.5);
}