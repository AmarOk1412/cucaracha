// TODO get from config pin
pub enum Gpio {
    P9_12=60,
    P9_14=50,
    P9_16=51,
    P9_18=4,
    P9_22=2,
}

pub fn GpioToPwm(gpio: Gpio) -> Option<(u32, u32)> {
    match gpio {
        Gpio::P9_14 => Some((4, 0)),
        Gpio::P9_22 => Some((0, 0)),
        _ => None,
    }
}