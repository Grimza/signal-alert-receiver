mod audio;
mod bootstrap;
mod platform;

use std::io::{BufRead, BufReader};
use std::thread;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

fn main() {
    let is_active = Arc::new(AtomicBool::new(true));
    let serial_active = is_active.clone();

    thread::spawn(move || {
        platform::windows::audio::init().expect("Failed to initialize audio system");

        let port_name = "COM5";
        let baud_rate = 115200;

        let port = bootstrap::serial::open_serial_port(port_name, baud_rate)
            .expect("Cannot open serial port");

        println!("Connected to {}", port_name);

        let mut reader = BufReader::new(port);

        loop {
            let mut line = String::new();

            if reader.read_line(&mut line).is_ok() {
                let message = line.trim();

                if message == "BEEP" {
                    if serial_active.load(Ordering::Relaxed) {
                        println!("Signal received: {}", message);
                        platform::windows::audio::set_master_volume(1.0)
                            .expect("Failed to adjust volume");
                        audio::player::play_sound("./src/assets/sounds/beep.mp3");
                    }
                }
            }
        }
    });

    bootstrap::ui::run_tray(is_active);
}
