use std::sync::{ Arc, Mutex };

use crate::beaglebone::*;
use crate::pin::*;

pub struct PwmLed {
    pub beagle: Arc<Mutex<BeagleBone>>,
    pub gpio: Gpio
}

impl PwmLed {
    pub fn new(beagle: Arc<Mutex<BeagleBone>>, gpio: Gpio) -> PwmLed {
        PwmLed::new_with_luminosity(beagle, gpio, 1.0)
    }

    pub fn new_with_luminosity(beagle: Arc<Mutex<BeagleBone>>, gpio: Gpio, mut luminosity: f32) -> PwmLed {
        let frequency = 20000;
        if luminosity > 1.0 {
            luminosity = 1.0;
        } else if luminosity < 0.0 {
            luminosity = 0.0;
        }
        beagle.clone().lock().unwrap().start_pwm(&gpio, (luminosity * frequency as f32) as u32, frequency);
        PwmLed {
            beagle,
            gpio
        }
    }

    pub fn set_luminosity(&self, mut luminosity: f32) -> bool {
        if luminosity > 1.0 {
            luminosity = 1.0;
        } else if luminosity < 0.0 {
            luminosity = 0.0;
        }
        let beagle = &mut self.beagle.lock().unwrap();
        let pwm = beagle.exported_pwms.get_mut(&self.gpio).unwrap();
        let duty_cycle = pwm.get_period_ns() as f32 * luminosity;
        pwm.set_duty_ns(duty_cycle as u32)
    }


    pub fn fade_to(&self, mut luminosity: f32, duration_ms: u32, update_period_ms: u32) -> bool {
        let mut step = duration_ms / update_period_ms;
        if step == 0 {
            step = 1;
        }
        if luminosity > 1.0 {
            luminosity = 1.0;
        } else if luminosity < 0.0 {
            luminosity = 0.0;
        }
        let beagle = &mut self.beagle.lock().unwrap();
        let pwm = beagle.exported_pwms.get_mut(&self.gpio).unwrap();

        let frequency = pwm.get_period_ns() as f32;
        let current_cycle: f32 = pwm.get_duty_ns() as f32;
        let wanted_cycle: f32 = luminosity * frequency;
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


    pub fn blink(&self, mut proportion: f32, speed: u32) -> bool {
        if proportion > 1.0 {
            proportion = 1.0;
        } else if proportion < 0.0 {
            proportion = 0.0;
        }

        let duty_cycle = (proportion * speed as f32) as u32;
        let beagle = &mut self.beagle.lock().unwrap();
        let pwm = beagle.exported_pwms.get_mut(&self.gpio).unwrap();
        if !pwm.set_period_ns(speed) {
            return false;
        }
        pwm.set_duty_ns(duty_cycle)
    }

    pub fn get_luminosity(&self) -> f32 {
        let beagle = &mut self.beagle.lock().unwrap();
        let pwm = beagle.exported_pwms.get_mut(&self.gpio).unwrap();
        let frequency = pwm.get_period_ns() as f32;
        let duty_cycle = pwm.get_duty_ns() as f32;
        duty_cycle / frequency
    }

}