use obs::{io::{audio_out::AudioOut, player::{Player, PlayerKind}}, synth::voice::Voice};

fn main() {
    let mut player = Player::new(PlayerKind::KeyboardPlayer);
    player.keyboard_player();
    let mut output = AudioOut::new();
    let mut voice = Voice::new(440, 0.5);
}