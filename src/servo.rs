
use std::sync::{Arc, Mutex};

use crate::beaglebone::*;
use crate::pin::*;

pub struct Servo {
    pub gpio: Gpio,
    pub beagle: Arc<Mutex<BeagleBone>>,
    pub degrees: f32,
    pub period: u32,
    pub min_duty: u32,
    pub max_duty: u32,
}

impl Servo {
    pub fn new(beagle: Arc<Mutex<BeagleBone>>, gpio: Gpio, degrees: f32) -> Servo {
        Servo::new_with_position(beagle, gpio, degrees, 0.0)
    }

    pub fn new_with_position(beagle: Arc<Mutex<BeagleBone>>, gpio: Gpio, degrees: f32, position: f32) -> Servo {
        // TODO, do we want to configure this?
        let frequency = 60; // Hz
        let period: u32 = ((1.0 / frequency as f32) * 1000000000 as f32 /* ns */) as u32;
        let min_duty = 500000 /* 0.5 ms */;
        let max_duty = 2500000 /* 2.5 ms */;

        let mut position = position;
        if position > degrees {
            position = degrees;
        } else if position < 0.0 {
            position = 0.0;
        }
        let duty = min_duty as f32 + (max_duty - min_duty) as f32 * (position / degrees);
        beagle.clone().lock().unwrap().start_pwm(&gpio, duty as u32, period);
        Servo {
            gpio,
            beagle,
            degrees,
            period,
            min_duty,
            max_duty
        }
    }

    pub fn set_position(&self, mut position: f32) -> bool {
        if position > self.degrees {
            position = self.degrees;
        } else if position < 0.0 {
            position = 0.0;
        }
        let beagle = &mut self.beagle.lock().unwrap();
        let pwm = beagle.exported_pwms.get_mut(&self.gpio).unwrap();
        let duty_cycle = self.min_duty as f32 + ((self.max_duty - self.min_duty) as f32 * (position / self.degrees));
        pwm.set_duty_ns(duty_cycle as u32)
    }


    pub fn go_to(&self, mut position: f32, duration_ms: u32, update_period_ms: u32) -> bool {
        let mut step = duration_ms / update_period_ms;
        if step == 0 {
            step = 1;
        }
        if position > self.degrees {
            position = self.degrees;
        } else if position < 0.0 {
            position = 0.0;
        }

        let beagle = &mut self.beagle.lock().unwrap();
        let pwm = beagle.exported_pwms.get_mut(&self.gpio).unwrap();
        let current_cycle: f32 = pwm.get_duty_ns() as f32;
        let wanted_cycle: f32 = self.min_duty as f32 + ((self.max_duty - self.min_duty) as f32 * (position / self.degrees));
        let inc = (wanted_cycle - current_cycle) / step as f32;
        for _ in 0..step {
            let mut new_cycle = pwm.get_duty_ns() as i32 + inc as i32;
            if new_cycle < 0 {
                new_cycle = 0;
            }
            let new_cycle = new_cycle as u32;
            if !pwm.set_duty_ns(new_cycle) {
                return false;
            }
            std::thread::sleep(std::time::Duration::from_millis(update_period_ms as u64));
        }

        true
    }

}