extern crate env_logger;
#[macro_use]
extern crate log;
extern crate sysfs_gpio;

pub mod beaglebone;
pub mod pin;
pub mod servo;
pub mod gpioled;
pub mod pwmled;
pub mod rgbled;

use beaglebone::*;
use pin::Gpio;
use gpioled::*;
use pwmled::*;
use rgbled::*;
use servo::*;
use std::{thread, time};

fn main() {
    // Init logging
    env_logger::init();

    println!("La cucaracha, la cucaracha,\nYa no puede caminar");

    //let mut pl = PwmLed::new(Gpio::P9_14);
    //pl.set_luminosity(1.0);
    //thread::sleep(time::Duration::from_secs(1));
    //pl.set_luminosity(0.5436);
    //thread::sleep(time::Duration::from_secs(1));
    //pl.set_luminosity(0.0);
    //pl.fade_to(1.0, /* duration */ 5000 /* ms */, /* update every */ 100 /* ms*/);
    //pl.fade_to(0.1, /* duration */ 5000 /* ms */, /* update every */ 1000 /* ms*/);
    //pl.blink(/* proportion */ 0.5, /* nHz  */ 100000000);

    //let mut servo = Servo::new(Gpio::P9_14, 180.0);
    //thread::sleep(time::Duration::from_secs(4));
    //servo.set_position(0.0);
    //thread::sleep(time::Duration::from_secs(4));
    //servo.set_position(50.0);
    //thread::sleep(time::Duration::from_secs(4));
    //servo.set_position(180.0);
    //thread::sleep(time::Duration::from_secs(4));
    //servo.go_to(0.0, /* duration */ 5000 /* ms */, /* update every */ 100 /* ms*/);
    //servo.go_to(75.0, /* duration */ 5000 /* ms */, /* update every */ 100 /* ms*/);

    let mut rgbled = RGBLed::new_with_color(
        (Gpio::P9_22, Gpio::P9_16, Gpio::P9_14),
        RGBLed::color_code_to_luminosity(255, 0, 0, 255));
    println!("Red");
    thread::sleep(time::Duration::from_secs(5));
    rgbled.set_color(RGBLed::color_code_to_luminosity(0, 255, 0, 255));
    println!("Green");
    thread::sleep(time::Duration::from_secs(5));
    rgbled.set_color(RGBLed::color_code_to_luminosity(0, 0, 255, 255));
    println!("Blue");
    thread::sleep(time::Duration::from_secs(5));
    rgbled.set_color(RGBLed::color_code_to_luminosity(128, 0, 128, 255));
    println!("Mid Purple");
    thread::sleep(time::Duration::from_secs(5));
    println!("Fade to red");
    rgbled.fade_to(RGBLed::color_code_to_luminosity(255, 0, 0, 255),
        /* duration */ 5000 /* ms */, /* update every */ 10 /* ms*/);
    println!("Fade to green");
    rgbled.fade_to(RGBLed::color_code_to_luminosity(0, 255, 0, 255),
        /* duration */ 5000 /* ms */, /* update every */ 10 /* ms*/);
    println!("Fade to blue");
    rgbled.fade_to(RGBLed::color_code_to_luminosity(0, 0, 255, 255),
        /* duration */ 5000 /* ms */, /* update every */ 10 /* ms*/);
    println!("Fade to white");
    rgbled.fade_to(RGBLed::color_code_to_luminosity(255, 255, 255, 255),
        /* duration */ 5000 /* ms */, /* update every */ 10 /* ms*/);
    println!("Fade to red");
    rgbled.fade_to(RGBLed::color_code_to_luminosity(255, 0, 0, 255),
        /* duration */ 5000 /* ms */, /* update every */ 10 /* ms*/);
    rgbled.blink(/* proportion */ 0.5, /* nHz  */ 100000000);
    println!("Blink");
}
