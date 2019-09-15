
use crate::sysfs_pwm::Pwm;
use crate::pin::*;

pub struct PwmLed {
    pub pwm: Pwm,
}

impl PwmLed {
    pub fn new(gpio: Gpio) -> PwmLed {
        PwmLed::new_with_luminosity(gpio, 1.0)
    }

    pub fn new_with_luminosity(gpio: Gpio, mut luminosity: f32) -> PwmLed {
        let pwm = GpioToPwm(gpio).unwrap();
        let pwm = Pwm::new(pwm.0, pwm.1).unwrap(); // number depends on chip, etc.
        match pwm.export() {
            Ok(()) => info!("PWM exported!"),
            Err(err) => error!("PWM could not be exported: {}", err),
        };
        pwm.enable(false).unwrap();
        let frequency = 20000;
        if luminosity > 1.0 {
            luminosity = 1.0;
        } else if luminosity < 0.0 {
            luminosity = 0.0;
        }
        let duty = frequency as f32 * (1.0 - luminosity);
        match pwm.set_duty_cycle_ns(duty as u32) {
            Err(e) => {println!("{}", e);}
            Ok(_) => {}
        };
        match pwm.set_period_ns(frequency) {
            Err(e) => {println!("{}", e);}
            Ok(_) => {}
        };
        pwm.enable(true).unwrap();
        PwmLed {
            pwm,
        }
    }

    pub fn set_luminosity(&self, mut luminosity: f32) -> bool {
        if luminosity > 1.0 {
            luminosity = 1.0;
        } else if luminosity < 0.0 {
            luminosity = 0.0;
        }
        let duty_cycle = self.pwm.get_period_ns().unwrap_or(0) as f32 * (1.0 - luminosity);
        let res = match self.pwm.set_duty_cycle_ns(duty_cycle as u32) {
            Err(e) => {
                println!("{}", e);
                false
            }
            Ok(_) => {
                true
            }
        };
        res
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
        let frequency = self.pwm.get_period_ns().unwrap_or(0) as f32;

        let current_cycle: f32 = self.pwm.get_duty_cycle_ns().unwrap_or(0) as f32;
        let wanted_cycle: f32 = (1.0 - luminosity) * frequency;
        let inc = (wanted_cycle - current_cycle) / step as f32;
        for _ in 0..step {
            let mut new_cycle = self.pwm.get_duty_cycle_ns().unwrap_or(0) as i32 + inc as i32;
            if new_cycle < 0 {
                new_cycle = 0;
            }
            let new_cycle = new_cycle as u32;
            if !self.pwm.set_duty_cycle_ns(new_cycle).is_ok() {
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

        let duty_cycle = ((1.0 - proportion) * speed as f32) as u32;
        self.pwm.enable(false).unwrap();
        let min_freq = std::cmp::min(speed, self.pwm.get_period_ns().unwrap_or(0));
        let change_duty = self.pwm.get_duty_cycle_ns().unwrap_or(0) > min_freq;
        // Avoid OS error
        if change_duty {
            let res = match self.pwm.set_duty_cycle_ns(min_freq) {
                Err(e) => {
                    false
                }
                Ok(_) => {
                    true
                }
            };
            if !res {
                self.pwm.enable(true).unwrap();
                return false;
            }
        }
        // Change freq
        let res = match self.pwm.set_period_ns(speed) {
            Err(e) => {
                false
            }
            Ok(_) => {
                true
            }
        };
        if !res {
            self.pwm.enable(true).unwrap();
            return false;
        }
        // And fill with blink
        let res = match self.pwm.set_duty_cycle_ns(duty_cycle) {
            Err(e) => {
                false
            }
            Ok(_) => {
                true
            }
        };
        self.pwm.enable(true).unwrap();
        res
    }

}