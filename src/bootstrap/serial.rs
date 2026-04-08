use std::time::Duration;

pub fn open_serial_port(port_name: &str, baud_rate: u32) -> serialport::Result<Box<dyn serialport::SerialPort>> {
    serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(1000))
        .open()
}
