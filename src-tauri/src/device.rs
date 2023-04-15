use std::io::{BufRead, BufReader};
use std::time::Duration;
use serialport::SerialPort;
use serialport::SerialPortType::UsbPort;

pub fn get_available_devices() -> Vec<String> {
    let mut ports = Vec::new();

    for port in serialport::available_ports().unwrap() {
        match port.port_type {
            UsbPort(t) => ports.push(port.port_name),
            _ => (),
        }
    }

    ports
}

pub fn connect_to_device(port_name: String) -> Box<dyn SerialPort> {
    serialport::new(port_name, 115_200)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open port")
}

pub fn read_data(port: Box<dyn SerialPort>) {
    let mut reader = BufReader::new(port);

    let mut my_str = String::new();

    loop {
        match reader.read_line(&mut my_str) {
            Ok(t) => println!("{}", my_str),
            Err(e) => println!("{}", e),
        }
    }
}

pub async fn process_data() {
    let mut ports = get_available_devices();

    if ports.len() > 0 {
        let mut port = connect_to_device(ports.pop().unwrap());
        read_data(port);
    }
}
