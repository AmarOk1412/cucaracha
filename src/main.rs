extern crate env_logger;
#[macro_use]
extern crate log;
extern crate sysfs_gpio;

pub mod pin;
pub mod servo;
pub mod gpioled;
pub mod pwmled;
pub mod sysfs_pwm;

use pin::Gpio;
use gpioled::*;
use pwmled::*;
use servo::*;
use std::{thread, time};

fn main() {
    // Init logging
    env_logger::init();

    println!("La cucaracha, la cucaracha,\nYa no puede caminar");

    //let pl = PwmLed::new(Gpio::P9_14);
    //pl.set_luminosity(1.0);
    //thread::sleep(time::Duration::from_secs(1));
    //pl.set_luminosity(0.5436);
    //thread::sleep(time::Duration::from_secs(1));
    //pl.set_luminosity(0.0);
    //pl.fade_to(1.0, /* duration */ 5000 /* ms */, /* update every */ 100 /* ms*/);
    //pl.fade_to(0.1, /* duration */ 5000 /* ms */, /* update every */ 1000 /* ms*/);
    //pl.blink(/* proportion */ 0.5, /* nHz  */ 100000000);

    let servo = Servo::new(Gpio::P9_14, 180.0);
    thread::sleep(time::Duration::from_secs(4));
    servo.set_position(0.0);
    thread::sleep(time::Duration::from_secs(4));
    servo.set_position(50.0);
    thread::sleep(time::Duration::from_secs(4));
    servo.set_position(180.0);
    thread::sleep(time::Duration::from_secs(4));
    servo.go_to(0.0, /* duration */ 5000 /* ms */, /* update every */ 100 /* ms*/);
    servo.go_to(75.0, /* duration */ 5000 /* ms */, /* update every */ 100 /* ms*/);
}
