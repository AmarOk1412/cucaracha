use std::sync::{Arc, Mutex};

use crate::beaglebone::*;
use crate::pin::*;
use crate::pwmled::*;

pub struct RGBLed {
    pub r_led: PwmLed,
    pub g_led: PwmLed,
    pub b_led: PwmLed,
}

impl RGBLed {
    pub fn new(beagle: Arc<Mutex<BeagleBone>>, (r_gpio, g_gpio, b_gpio) : (Gpio, Gpio, Gpio)) -> RGBLed {
        RGBLed::new_with_color(beagle, (r_gpio, g_gpio, b_gpio), (1.0, 1.0, 1.0))
    }

    pub fn new_with_color(beagle: Arc<Mutex<BeagleBone>>,
        (r_gpio, g_gpio, b_gpio) : (Gpio, Gpio, Gpio),
        (mut r, mut g, mut b): (f32, f32, f32)) -> RGBLed {
        RGBLed {
            r_led: PwmLed::new_with_luminosity(beagle.clone(), r_gpio, r),
            g_led: PwmLed::new_with_luminosity(beagle.clone(), g_gpio, g),
            b_led: PwmLed::new_with_luminosity(beagle.clone(), b_gpio, b),
        }
    }

    pub fn color_code_to_luminosity(mut r: u32, mut g: u32, mut b: u32, mut a: u32) -> (f32, f32, f32) {
        // TODO utils to check bound, same for pwmled
        if r > 255 {
            r = 255;
        }
        if g > 255 {
            g = 255;
        }
        if b > 255 {
            b = 255;
        }
        if a > 255 {
            a = 255;
        }
        ((r * a) as f32 / 255.0, (g * a) as f32 / 255.0, (b * a) as f32 / 255.0)
    }

    pub fn set_color(&self, (r, g, b): (f32, f32, f32)) -> bool {
        let res = self.r_led.set_luminosity(r);
        if !res {
            return false;
        }
        let res = self.g_led.set_luminosity(g);
        if !res {
            return false;
        }
        let res = self.b_led.set_luminosity(b);
        res
    }


    pub fn fade_to(&self, (mut r, mut g, mut b): (f32, f32, f32), duration_ms: u32, update_period_ms: u32) -> bool {
        let mut step = duration_ms / update_period_ms;
        if step == 0 {
            step = 1;
        }
        if r > 1.0 {
            r = 1.0;
        } else if r < 0.0 {
            r = 0.0;
        }
        let current_r = self.r_led.get_luminosity();
        let inc_r = (r - current_r) / step as f32;
        if g > 1.0 {
            g = 1.0;
        } else if g < 0.0 {
            g = 0.0;
        }
        let current_g = self.g_led.get_luminosity();
        let inc_g = (g - current_g) / step as f32;
        if b > 1.0 {
            b = 1.0;
        } else if b < 0.0 {
            b = 0.0;
        }
        let current_b = self.b_led.get_luminosity();
        let inc_b = (b - current_b) / step as f32;

        for _ in 0..step {
            if !self.r_led.set_luminosity(self.r_led.get_luminosity() + inc_r) {
                break;
            }
            if !self.g_led.set_luminosity(self.g_led.get_luminosity() + inc_g) {
                break;
            }
            if !self.b_led.set_luminosity(self.b_led.get_luminosity() + inc_b) {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(update_period_ms as u64));
        }

        true
    }


    pub fn blink(&self, proportion: f32, speed: u32) -> bool {
        let res = self.r_led.blink(proportion, speed);
        if !res {
            return false;
        }
        let res = self.g_led.blink(proportion, speed);
        if !res {
            return false;
        }
        let res = self.b_led.blink(proportion, speed);
        res
    }

}