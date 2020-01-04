
use crate::maestro::*;
use crate::pin::*;
use std::sync::{Arc, Mutex};

pub struct PwmServoSettings {
    pub pwm: Pwm,
    pub degrees: f32,
    pub period: u32,
    pub min_duty: u32,
    pub max_duty: u32,
}

pub struct MaestroServoSettings {
    pub servo_range: f32,
    pub channel: u8,
    pub maestro: Arc<Mutex<Maestro>>
}

pub struct Servo {
    pwm_settings: Option<PwmServoSettings>,
    maestro_settings: Option<MaestroServoSettings>,
}

impl Servo {
    /**
     * Create a new PWM servo
     */
    pub fn new(gpio: Gpio, degrees: f32) -> Servo {
        Servo::new_with_position(gpio, degrees, 0.0)
    }

    /**
     * Create a new PWM servo with a given position
     */
    pub fn new_with_position(gpio: Gpio, degrees: f32, position: f32) -> Servo {
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
        let pwm = gpio_to_pwm(&gpio);
        if pwm.is_none() {
            panic!("Invalid PWM");
        }
        let mut pwm = pwm.unwrap();
        pwm.start_pwm(duty as u32, period);
        Servo {
            pwm_settings: Some(PwmServoSettings {
                pwm,
                degrees,
                period,
                min_duty,
                max_duty
            }),
            maestro_settings: None
        }
    }

    /**
     * Create a new Servo linked to a Maestro board
     */
    pub fn new_from_maestro(servo_range: f32, channel: u8, maestro: Arc<Mutex<Maestro>>) -> Servo {
        Servo {
            pwm_settings: None,
            maestro_settings: Some(MaestroServoSettings {
                servo_range,
                channel,
                maestro
            })
        }
    }

     /**
     * Change a servo position
     * @param position     wanted position in servo_range
     * @return if the operation was successful
     */
    pub fn set_position(&mut self, mut position: f32) -> bool {
        if self.pwm_settings.is_some() {
            let settings = self.pwm_settings.as_mut().unwrap();
            if position > settings.degrees {
                position = settings.degrees;
            } else if position < 0.0 {
                position = 0.0;
            }
            let duty_cycle = settings.min_duty as f32 + ((settings.max_duty - settings.min_duty) as f32 * (position / settings.degrees));
            return settings.pwm.set_duty_ns(duty_cycle as u32);
        }
        let settings = self.maestro_settings.as_ref().unwrap();
        let mut position = position;
        if position > settings.servo_range {
            position = settings.servo_range;
        }
        let pos_to_target = position as f32 / settings.servo_range;
        let mut maestro = settings.maestro.lock().unwrap();
        let target = maestro.min_target + ((maestro.max_target - maestro.min_target) as f32 * pos_to_target) as u16;
        maestro.set_target(settings.channel, target)
    }

    /**
     * Get current position returned by the beagle
     * @note a difference will exists between real value and what the Maestro send
     * @return the position of the servo
     */
    pub fn get_position(&mut self) -> u16 {
        if self.maestro_settings.is_none() {
            let settings = self.pwm_settings.as_ref().unwrap();
            let current_cycle: f32 = settings.pwm.get_duty_ns() as f32;
            let current_pos = current_cycle / (settings.max_duty - settings.min_duty) as f32;
            return current_pos as u16;
        }
        let settings = self.maestro_settings.as_ref().unwrap();
        let mut maestro = settings.maestro.lock().unwrap();
        let mut pos = maestro.get_target(settings.channel);
        pos = (((pos as f32  - maestro.min_target as f32) / (maestro.max_target as f32 - maestro.min_target as f32)) * settings.servo_range) as u16;
        pos
    }

    /**
     * Go to a given position in the duration given, with update_period
     * @param position          Wanted position
     * @param duration_ms       Wanted time for the transition
     * @param update_period_ms  Period beetween steps
     * @return if the operation was successful
     */
    pub fn go_to(&mut self, mut position: f32, duration_ms: u32, update_period_ms: u32) -> bool {
        let mut step = duration_ms / update_period_ms;
        if step == 0 {
            step = 1;
        }
        if self.pwm_settings.is_some() {
            let settings = self.pwm_settings.as_ref().unwrap();
            if position > settings.degrees {
                position = settings.degrees;
            } else if position < 0.0 {
                position = 0.0;
            }
        } else {
            let settings = self.maestro_settings.as_ref().unwrap();
            if position > settings.servo_range as f32 {
                position = settings.servo_range as f32;
            } else if position < 0.0 {
                position = 0.0;
            }
        }

        let inc;
        let current_position;
        if self.pwm_settings.is_some() {
            let settings = self.pwm_settings.as_ref().unwrap();
            let current_cycle: f32 = settings.pwm.get_duty_ns() as f32;
            let wanted_cycle: f32 = settings.min_duty as f32 + ((settings.max_duty - settings.min_duty) as f32 * (position / settings.degrees));
            inc = (wanted_cycle - current_cycle) / step as f32;
            current_position = current_cycle / (settings.max_duty - settings.min_duty) as f32;
        } else {
            current_position = self.get_position() as f32;
            let wanted_postion = position as f32;
            inc = (wanted_postion - current_position) / step as f32;
        }

        for s in 0..step {
            if self.pwm_settings.is_some() {
                let settings = self.pwm_settings.as_mut().unwrap();
                let mut new_cycle = settings.pwm.get_duty_ns() as i32 + inc as i32;
                if new_cycle < 0 {
                    new_cycle = 0;
                }
                let new_cycle = new_cycle as u32;
                if !settings.pwm.set_duty_ns(new_cycle) {
                    return false;
                }
            } else {
                let new_pos = current_position + inc * s as f32;
                if !self.set_position(new_pos) {
                    return false;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(update_period_ms as u64));
        }

        true
    }

}