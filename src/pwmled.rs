
use crate::sysfs_pwm::{Pwm, Result};
use crate::pin::*;

pub struct PwmLed {
    pub pwm: Pwm,
}

/*

fn pwm_increase_to_max(pwm: &Pwm, duration_ms: u32, update_period_ms: u32) -> Result<()> {
    let step: f32 = duration_ms as f32 / update_period_ms as f32;
    let mut duty_cycle = 0.0;
    let period_ns: u32 = pwm.get_period_ns()?;
    while duty_cycle < 1.0 {
        pwm.set_duty_cycle_ns((duty_cycle * period_ns as f32) as u32)?;
        duty_cycle += step;
    }
    pwm.set_duty_cycle_ns(period_ns)
}

fn pwm_decrease_to_minimum(pwm: &Pwm, duration_ms: u32, update_period_ms: u32) -> Result<()> {
    let step: f32 = duration_ms as f32 / update_period_ms as f32;
    let mut duty_cycle = 1.0;
    let period_ns: u32 = pwm.get_period_ns()?;
    while duty_cycle > 0.0 {
        pwm.set_duty_cycle_ns((duty_cycle * period_ns as f32) as u32)?;
        duty_cycle -= step;
    }
    pwm.set_duty_cycle_ns(0)
}*/

impl PwmLed {
    pub fn new(gpio: Gpio) -> PwmLed {
        let pwm = GpioToPwm(gpio).unwrap();
        let pwm = Pwm::new(pwm.0, pwm.1).unwrap(); // number depends on chip, etc.
        match pwm.export() {
            Ok(()) => info!("PWM exported!"),
            Err(err) => error!("PWM could not be exported: {}", err),
        };
        pwm.enable(true).unwrap();
        PwmLed {
            pwm
        }
    }

    pub fn set_frequency(&self, freq: u32) {
        self.pwm.set_period_ns(freq);
    }

    pub fn get_frequency(&self) -> u32 {
        self.pwm.get_period_ns().unwrap_or(0)
    }

    pub fn set_duty_cycle(&self, dc: u32) {
        self.pwm.set_duty_cycle_ns(dc);
    }

    pub fn get_duty_cycle(&self) -> u32 {
        self.pwm.get_duty_cycle_ns().unwrap_or(0)
    }
}