
use std::io::prelude::*;
use serial::prelude::*;

pub struct Settings {
    pub port: String,
    pub device: u8
}

pub struct Maestro {
    port: Box<dyn SerialPort>,
    device: u8
}

impl Maestro {
    pub fn new() -> Maestro {
        Maestro::new_with_settings(Settings {
            port: String::from("/dev/ttyACM0"),
            device: 0x0c
        })
    }

    pub fn new_with_settings(settings: Settings) -> Maestro {
        let mut port = serial::open(&settings.port).unwrap();
        let port_settings: serial::PortSettings = serial::PortSettings {
            baud_rate:     serial::Baud9600,
            char_size:     serial::Bits8,
            parity:        serial::ParityNone,
            stop_bits:     serial::Stop1,
            flow_control:  serial::FlowNone,
        };
        port.configure(&port_settings).ok().expect("Cannot configure Maestro");
        Maestro {
            port: Box::new(port),
            device: settings.device
        }
    }

    fn send(&mut self, data: &mut Vec<u8>) -> bool {
        let mut buf = vec![0xaa, self.device];
        buf.append(data);
        debug!("send {:?}", data);
        self.port.write(&data[..]).is_ok()
    }

    pub fn set_target(&mut self, channel: u8, target: u16) -> bool {
        let lsb = (target & 0x7f) as u8;
        let msb = (target >> 7 & 0x7f) as u8;
        let mut data = vec![0x04, channel, lsb, msb];
        self.send(&mut data)
    }
}