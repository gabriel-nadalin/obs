use obs::{io::player::{Player, PlayerKind}, synth::channel::NoiseChannel, AMPLITUDE_MAX};

fn main() {
    let mut noise = NoiseChannel::new(0.3);
    let mut player = Player::new(PlayerKind::KeyboardPlayer);
    let buffer = NoiseChannel::generate_brown_noise(300000, 0.005);
    for sample in buffer {
        // player.audio_out(sample == AMPLITUDE_MAX);
        // dbg!(sample == AMPLITUDE_MAX);
    }
    player.drain();

    // loop {
    //     player.audio_out(noise.get_sample());
    // }




    // initialization

    //while !end {
        // if key_pressed {
            // key_press()
        // audio_out()
        // update()
}