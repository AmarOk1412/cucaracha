
use crate::sysfs_pwm::Pwm;
use crate::pin::*;

pub struct Servo {
    pub pwm: Pwm,
    pub degrees: f32,
    pub period: u32,
    pub min_duty: u32,
    pub max_duty: u32,
}

impl Servo {
    pub fn new(gpio: Gpio, degrees: f32) -> Servo {
        Servo::new_with_position(gpio, degrees, 0.0)
    }

    pub fn new_with_position(gpio: Gpio, degrees: f32, position: f32) -> Servo {
        let pwm = GpioToPwm(gpio).unwrap();
        let pwm = Pwm::new(pwm.0, pwm.1).unwrap(); // number depends on chip, etc.
        match pwm.export() {
            Ok(()) => info!("PWM exported!"),
            Err(err) => error!("PWM could not be exported: {}", err),
        };

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
        pwm.enable(false).unwrap();
        let duty = min_duty as f32 + (max_duty - min_duty) as f32 * (position / degrees);
        match pwm.set_duty_cycle_ns(duty as u32) {
            Err(e) => {println!("{}", e);}
            Ok(_) => {}
        };
        match pwm.set_period_ns(period) {
            Err(e) => {println!("{}", e);}
            Ok(_) => {}
        };
        pwm.enable(true).unwrap();
        Servo {
            pwm,
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
        let duty_cycle = self.min_duty as f32 + ((self.max_duty - self.min_duty) as f32 * (position / self.degrees));
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

        let current_cycle: f32 = self.pwm.get_duty_cycle_ns().unwrap_or(0) as f32;
        let wanted_cycle: f32 = self.min_duty as f32 + ((self.max_duty - self.min_duty) as f32 * (position / self.degrees));
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

}