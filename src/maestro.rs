
use std::io::prelude::*;
use serial::prelude::*;

/**
 * Represent what the end user see. servo_range should be the most interesting
 * setting to change.
 */
pub struct MaestroSettings {
    pub port: String,
    pub device: u8,
    pub min_target: u16,
    pub max_target: u16,
    pub servo_range: u16,
}

/**
 * Represent the Maestro board
 * Tested with a mini maestro 18
 */
pub struct Maestro {
    port: Box<dyn SerialPort>,
    device: u8,
    min_target: u16,
    max_target: u16,
    servo_range: u16,
}

impl Maestro {
    /**
     * Get a new maestro instance, with default settings
     * port: /dev/ttyACM0,
     * device: 0x0c
     * min_target: 2000
     * max_target: 10000
     * servo_range: 180
     * @return the new Maestro's instance
     */
    pub fn new() -> Maestro {
        Maestro::new_with_settings(MaestroSettings {
            port: String::from("/dev/ttyACM0"),
            device: 0x0c,
            min_target: 2000,
            max_target: 10000,
            servo_range: 180,
        })
    }

    /**
     * Get a new maestro instance, with settings specified by the user
     * @param settings      User settings
     * @return the new Maestro's instance
     */
    pub fn new_with_settings(settings: MaestroSettings) -> Maestro {
        // Configure the serial port
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
            device: settings.device,
            min_target: settings.min_target,
            max_target: settings.max_target,
            servo_range: settings.servo_range,
        }
    }

    /**
     * Send a maestro command via serial
     * The pkt will be [0xaa, device, data...]
     * @param data      buffer to send
     * @return if the operation was successful
     */
    fn send(&mut self, data: &mut Vec<u8>) -> bool {
        let mut buf = vec![0xaa, self.device];
        buf.append(data);
        self.port.write(&buf[..]).is_ok()
    }

    /**
     * Change a channel target
     * @param channel      channel to configure
     * @param target       wanted target between min_target and max_target
     * @return if the operation was successful
     */
    pub fn set_target(&mut self, channel: u8, target: u16) -> bool {
        let mut target = target;
        if target < self.min_target {
            target = self.min_target;
        }
        if target > self.max_target {
            target = self.max_target;
        }
        let lsb = (target & 0x7f) as u8;
        let msb = (target >> 7 & 0x7f) as u8;
        let mut data = vec![0x04, channel, lsb, msb];
        self.send(&mut data)
    }


    /**
     * Change a channel position
     * @param channel      channel to configure
     * @param position     wanted position in servo_range
     * @return if the operation was successful
     */
    pub fn set_position(&mut self, channel: u8, position: u16) -> bool {
        let mut position = position;
        if position > self.servo_range {
            position = self.servo_range;
        }
        let pos_to_target = position as f32 / self.servo_range as f32;
        let target = self.min_target + ((self.max_target - self.min_target) as f32 * pos_to_target) as u16;
        self.set_target(channel, target)
    }

    /**
     * Change a channel speed
     * @param channel      channel to configure
     * @param speed        wanted speed
     * @return if the operation was successful
     */
    pub fn set_speed(&mut self, channel: u8, speed: u16) -> bool {
        let lsb = (speed & 0x7f) as u8;
        let msb = (speed >> 7 & 0x7f) as u8;
        let mut data = vec![0x07, channel, lsb, msb];
        self.send(&mut data)
    }

    /**
     * Change a channel accel
     * @param channel      channel to configure
     * @param accel        wanted accel
     * @return if the operation was successful
     */
    pub fn set_accel(&mut self, channel: u8, accel: u16) -> bool {
        let lsb = (accel & 0x7f) as u8;
        let msb = (accel >> 7 & 0x7f) as u8;
        let mut data = vec![0x09, channel, lsb, msb];
        self.send(&mut data)
    }

    /**
     * Get current position in servo_range
     * @param channel       Channel to read
     * @return the position read on the socket
     * @note a difference will exists between real value and what the Maestro send
     */
    pub fn get_position(&mut self, channel: u8) -> u16 {
        let mut data = vec![0x10, channel];
        self.send(&mut data);
        let mut buf = vec![0; 2];
        self.port.read(&mut buf[..]).ok().expect("Couldn't read on serial socket");
        let pos = ((buf[1] as u16 & 0x00ff) << 8) + buf[0] as u16;
        let mut pos = pos as f32 / self.max_target as f32;
        pos = pos * self.servo_range as f32;
        return pos as u16;
    }

    /**
     * Get if all servos reached their target
     * @return if all servos reached their target
     */
    pub fn is_moving(&mut self) -> bool {
        let mut data = vec![0x13];
        self.send(&mut data);
        let mut buf = vec![0; 1];
        self.port.read(&mut buf[..]).ok().expect("Couldn't read on serial socket");
        return buf[0] != 0;
    }
}