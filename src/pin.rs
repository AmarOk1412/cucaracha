// TODO get from config pin
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Gpio {
    P9_12=60,
    P9_14=50,
    P9_16=51,
    P9_18=4,
    P9_22=2,
}

pub struct Pwm {
    pub sysfs: u32,
    pub index: u32,
    pub chip: String,
    pub addr: String,
    pub key: String,
}

// Copied from https://github.com/jadonk/bonescript/blob/master/src/bone.js
pub fn GpioToPwm(gpio: &Gpio) -> Option<Pwm> {
    match gpio {
        Gpio::P9_14 => Some(Pwm {
            sysfs: 4,
            index: 0,
            chip: String::from("48302000"),
            addr: String::from("48302200"),
            key: String::from("P9_14")
        }),
        Gpio::P9_16 => Some(Pwm {
            sysfs: 4,
            index: 1,
            chip: String::from("48302000"),
            addr: String::from("48302200"),
            key: String::from("P9_16")
        }),
        Gpio::P9_22 => Some(Pwm {
            sysfs: 1,
            index: 0,
            chip: String::from("48300000"),
            addr: String::from("48300200"),
            key: String::from("P9_22")
        }),
        _ => None,
    }
}