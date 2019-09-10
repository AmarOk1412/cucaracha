use sysfs_gpio::{Direction, Pin};

use crate::pin::Gpio;

pub enum State {
    LOW = 0,
    HIGH = 1
}

pub struct GpioLed {
    pub pin: Pin,
    pub direction: Direction,
}

impl GpioLed {
    pub fn new(gpio: Gpio) -> GpioLed {
        let pin = Pin::new(gpio as u64);
        if !pin.is_exported() {
            match pin.export() {
                Ok(()) => info!("Gpio {} exported!", pin.get_pin()),
                Err(err) => error!("Gpio {} could not be exported: {}", pin.get_pin(), err),
            }
        }
        match pin.get_direction() {
            Ok(direction) => {
                if direction != Direction::Out {
                    pin.set_direction(Direction::Out).ok().expect("Cannot set direction for Gpio");
                }
            },
            Err(err) => error!("Gpio {} cannot get direction: {}", pin.get_pin(), err),
        }
        GpioLed {
            pin,
            direction: Direction::Out
        }
    }

    pub fn set_state(&self, state: State) -> bool {
        self.pin.set_value(state as u8).is_ok()
    }
}