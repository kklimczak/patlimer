use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::time::Duration;
use serialport::SerialPort;
use serialport::SerialPortType::UsbPort;
use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub enum Commands {
    StartRace
}

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

pub fn read_data(mut reader: BufReader<Box<dyn SerialPort>>) {
    let mut my_str = String::new();

    loop {
        match reader.read_line(&mut my_str) {
            Ok(t) => {
                let parts = my_str.split(":");
                let timestamp: Vec<&str> = parts.collect();
                if timestamp.len() > 0 {
                }
                println!("{}", my_str);
                my_str.clear();
            },
            Err(e) => println!("{}", e),
        }
    }
}

pub async fn process_data(mut commands_rx: Receiver<Commands>) {
    let mut ports = get_available_devices();

    if ports.len() > 0 {
        let mut port = connect_to_device(ports.pop().unwrap());
        let mut port_clone = port.try_clone().unwrap();

        tauri::async_runtime::spawn(async move {
            while let Some(command) = commands_rx.recv().await {
                match command {
                    Commands::StartRace => {
                        let output = "r:s\n".as_bytes();
                        port.write(output).expect("Writing failed.");
                    }
                }
                dbg!(command);
            }
        });

        tauri::async_runtime::spawn(async move {
            let mut reader = BufReader::new(port_clone);
            read_data(reader);
        });


    }
}
