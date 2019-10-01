
use std::io;
use std::time::Duration;

use std::io::prelude::*;
use serial::prelude::*;

pub struct Maestro {
    port: Box<dyn SerialPort>
}

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate:    serial::Baud9600,
    char_size:    serial::Bits8,
    parity:       serial::ParityNone,
    stop_bits:    serial::Stop1,
    flow_control: serial::FlowNone,
};

impl Maestro {
    pub fn new() -> Maestro {
        Maestro::new_with_port(String::from("/dev/ttyACM0"))
    }

    pub fn new_with_port(port: String) -> Maestro {
        let mut port = serial::open(&port).unwrap();
        port.configure(&SETTINGS).ok().expect("Cannot configure Maestro");
        Maestro {
            port: Box::new(port)
        }
    }

    pub fn set_target(&mut self, channel: u8, target: u16) {
        let lsb = (target & 0x7f) as u8;
        let msb = (target >> 7 & 0x7f) as u8;
        let data = vec![0xaa, 0x0c, 0x04, channel, lsb, msb];
        println!("{:?}", data);
        self.port.write(&data[..]); // TODO check ok
    }
}