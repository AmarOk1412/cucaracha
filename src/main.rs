extern crate env_logger;
#[macro_use]
extern crate log;
extern crate sysfs_gpio;

pub mod pin;
pub mod gpioled;
pub mod pwmled;
pub mod sysfs_pwm;

use pin::Gpio;
use gpioled::*;
use pwmled::*;
use std::{thread, time};

fn main() {
    // Init logging
    env_logger::init();

    println!("Hello, world!");

    let pl = PwmLed::new(Gpio::P9_14);
    //pl.set_frequency(20000);
    pl.set_frequency(10000000);

    //let l = GpioLed::new(Gpio::P9_12);
    //let l2 = GpioLed::new(Gpio::P9_18);
    //let mut i = 0;
    let mut i = 500000;
    let mut inc = true;
    loop {
        pl.set_duty_cycle(i);
        thread::sleep(time::Duration::from_nanos(100));
        if inc {
        //    i += 1;
            i += 100;
//            if i == 20000 {
            if i == 2500000 {
                inc = false;
            }
        } else {
//            i -= 1;
            i -= 100;
            //if i == 0 {
            if i == 500000 {
                inc = true;
            }
        }
        //l.set_state(State::HIGH);
        //l2.set_state(State::HIGH);
        //thread::sleep(time::Duration::from_secs(1));
        //l.set_state(State::LOW);
        //l2.set_state(State::LOW);
        //thread::sleep(time::Duration::from_secs(1));
    }
}
