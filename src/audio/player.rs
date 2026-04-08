use rodio::{Decoder, DeviceSinkBuilder, Player};
use std::fs::File;
use std::io::BufReader;

pub fn play_sound(path: &str) {
    let mut handle =
        DeviceSinkBuilder::open_default_sink().expect("Cannot access audio output device");
    handle.log_on_drop(false);

    let player = Player::connect_new(&handle.mixer());
    let file = File::open(path).expect("Audio file not found");
    let source = Decoder::try_from(BufReader::new(file)).unwrap();

    player.append(source);
    player.sleep_until_end();
}
