use crate::pin::*;

pub struct PwmLed {
    pub pwm: Pwm
}

impl PwmLed {
    pub fn new(gpio: Gpio) -> PwmLed {
        PwmLed::new_with_luminosity(gpio, 1.0)
    }

    pub fn new_with_luminosity(gpio: Gpio, mut luminosity: f32) -> PwmLed {
        let frequency = 20000;
        if luminosity > 1.0 {
            luminosity = 1.0;
        } else if luminosity < 0.0 {
            luminosity = 0.0;
        }
        let pwm = gpio_to_pwm(&gpio);
        if pwm.is_none() {
            panic!("Invalid PWM");
        }
        let mut pwm = pwm.unwrap();
        pwm.start_pwm((luminosity * frequency as f32) as u32, frequency);
        PwmLed {
            pwm
        }
    }

    pub fn set_luminosity(&mut self, mut luminosity: f32) -> bool {
        if luminosity > 1.0 {
            luminosity = 1.0;
        } else if luminosity < 0.0 {
            luminosity = 0.0;
        }
        let duty_cycle = self.pwm.get_period_ns() as f32 * luminosity;
        self.pwm.set_duty_ns(duty_cycle as u32)
    }


    pub fn fade_to(&mut self, mut luminosity: f32, duration_ms: u32, update_period_ms: u32) -> bool {
        let mut step = duration_ms / update_period_ms;
        if step == 0 {
            step = 1;
        }
        if luminosity > 1.0 {
            luminosity = 1.0;
        } else if luminosity < 0.0 {
            luminosity = 0.0;
        }

        let frequency = self.pwm.get_period_ns() as f32;
        let current_cycle: f32 = self.pwm.get_duty_ns() as f32;
        let wanted_cycle: f32 = luminosity * frequency;
        let inc = (wanted_cycle - current_cycle) / step as f32;

        for _ in 0..step {
            let mut new_cycle = self.pwm.get_duty_ns() as i32 + inc as i32;
            if new_cycle < 0 {
                new_cycle = 0;
            }
            let new_cycle = new_cycle as u32;
            if !self.pwm.set_duty_ns(new_cycle) {
                return false;
            }
            std::thread::sleep(std::time::Duration::from_millis(update_period_ms as u64));
        }

        true
    }


    pub fn blink(&mut self, mut proportion: f32, speed: u32) -> bool {
        if proportion > 1.0 {
            proportion = 1.0;
        } else if proportion < 0.0 {
            proportion = 0.0;
        }

        let duty_cycle = (proportion * speed as f32) as u32;
        if !self.pwm.set_period_ns(speed) {
            return false;
        }
        self.pwm.set_duty_ns(duty_cycle)
    }

    pub fn get_luminosity(&self) -> f32 {
        let frequency = self.pwm.get_period_ns() as f32;
        let duty_cycle = self.pwm.get_duty_ns() as f32;
        duty_cycle / frequency
    }

}