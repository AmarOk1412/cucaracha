use crate::pin::*;
use std::path::Path;
use std::fs;
use std::{thread, time};
use std::collections::HashMap;

pub struct Pwm {
    pub enable: bool,
    pub duty_ns: u32,
    pub period_ns: u32,
    pub info: PwmInfo,
}

impl Pwm {
    pub fn is_pwm_enable(gpio: &Gpio) -> bool {
        let pwm = GpioToPwmInfo(gpio);
        if pwm.is_none() {
            return false;
        }
        let pwm = pwm.unwrap();
        let enabled_path = format!("/sys/class/pwm/pwm-{}:{}/enable", pwm.sysfs, pwm.index);
        let enabled = fs::read_to_string(&enabled_path).unwrap_or(String::from("0"));
        enabled == "1"
    }

    pub fn is_enable(&self) -> bool {
        self.enable
    }

    pub fn set_enable(&mut self, enable: bool) -> bool {
        let enabled_path = format!("/sys/class/pwm/pwm-{}:{}/enable", self.info.sysfs, self.info.index);
        let ok = match enable {
            true => fs::write(&enabled_path, String::from("1")).is_ok(),
            _ => fs::write(&enabled_path, String::from("0")).is_ok()
        };
        if ok {
            self.enable = enable;
        }
        ok
    }

    pub fn get_pwm_duty_ns(gpio: &Gpio) -> u32 {
        let pwm = GpioToPwmInfo(gpio);
        if pwm.is_none() {
            return 0;
        }
        let pwm = pwm.unwrap();
        let duty_ns_path = format!("/sys/class/pwm/pwm-{}:{}/duty_cycle", pwm.sysfs, pwm.index);
        let duty_ns = fs::read_to_string(&duty_ns_path).unwrap_or(String::from("0"));
        duty_ns.parse().unwrap_or(0)
    }

    pub fn get_duty_ns(&self) -> u32 {
        self.duty_ns
    }

    pub fn get_pwm_period_ns(gpio: &Gpio) -> u32 {
        let pwm = GpioToPwmInfo(gpio);
        if pwm.is_none() {
            return 0;
        }
        let pwm = pwm.unwrap();
        let period_ns_path = format!("/sys/class/pwm/pwm-{}:{}/period", pwm.sysfs, pwm.index);
        let period_ns = fs::read_to_string(&period_ns_path).unwrap_or(String::from("0"));
        period_ns.parse().unwrap_or(0)
    }

    pub fn get_period_ns(&self) -> u32 {
        self.period_ns
    }

    pub fn set_duty_ns(&mut self, duty_ns: u32) -> bool {
        let duty_ns_path = format!("/sys/class/pwm/pwm-{}:{}/duty_cycle", self.info.sysfs, self.info.index);
        let ok = fs::write(&duty_ns_path, duty_ns.to_string()).is_ok();
        info!("Write {} in {} - {}", duty_ns, duty_ns_path, ok);
        if ok {
            self.duty_ns = duty_ns;
        }
        ok
    }

    pub fn set_period_ns(&mut self, period_ns: u32) -> bool {
        let period_ns_path = format!("/sys/class/pwm/pwm-{}:{}/period", self.info.sysfs, self.info.index);
        let new_duty_ns = (period_ns as f32 / self.period_ns as f32) * self.duty_ns as f32;
        if period_ns < self.period_ns {
            // Going to a shorter period, update duty_cycle first to avoid any error
            if !self.set_duty_ns(new_duty_ns as u32) {
                error!("Can't change duty_cycle");
                return false;
            }
        }
        // Update freq
        info!("Write {} in {}", period_ns, period_ns_path);
        if fs::write(&period_ns_path, period_ns.to_string()).is_ok() {
            self.period_ns = period_ns;
        } else {
            error!("Can't change period {}: {}", period_ns_path, period_ns);
            return false;
        }
        if period_ns > self.period_ns {
            // Update duty cycle to keep level
            if !self.set_duty_ns(new_duty_ns as u32) {
                error!("Can't change duty_cycle");
                return false;
            }
        }
        true
    }
}

// TODO remove this and only keep Pwm
pub struct BeagleBone {
    pub exported_pwms: HashMap<Gpio, Pwm>
}

impl BeagleBone {
    pub fn new() -> BeagleBone {
        BeagleBone {
            exported_pwms: HashMap::new(),
        }
    }

    fn pwm_setup(&mut self, gpio: &Gpio) -> bool {
        // For BBB SEEED, uboot is enabled (/bin/grep -c bone_capemgr.uboot_capemgr_enabled=1 /proc/cmdline)
        // So, there is no need to initialize pwm mode.
        let pwm = GpioToPwmInfo(gpio);
        if pwm.is_none() {
            return false;
        }
        let pwm = pwm.unwrap();
        // First, set the PIN in pwm mode
        let state_path = format!("/sys/devices/platform/ocp/ocp:{}_pinmux/state", pwm.key);
        fs::write(&state_path, "pwm").expect("Unable to write file");
        // Second, init the PIN state
        let pwm_path = format!("/sys/devices/platform/ocp/{}.epwmss/{}.pwm/pwm/pwmchip{}",
            pwm.chip, pwm.addr, pwm.sysfs);
        let exported_path = format!("{}/pwm-{}:{}", pwm_path, pwm.sysfs, pwm.index);
        if !Path::new(&exported_path).exists() {
            // Export pin
            let export_path = format!("{}/export", pwm_path);
            fs::write(&export_path, pwm.index.to_string()).expect("Unable to write file");
            thread::sleep(time::Duration::from_millis(100));
        }
        self.exported_pwms.insert(gpio.clone(), Pwm {
            enable: Pwm::is_pwm_enable(gpio),
            duty_ns: Pwm::get_pwm_duty_ns(gpio),
            period_ns: Pwm::get_pwm_period_ns(gpio),
            info: pwm
        });
        true
    }

    pub fn start_pwm(&mut self, key: &Gpio, duty_ns: u32, period_ns: u32) -> bool {
        if !self.exported_pwms.contains_key(key) {
            if !self.pwm_setup(key) {
                error!("Can't setup pwm");
                return false;
            }
        }
        let pwm = self.exported_pwms.get_mut(key).unwrap();
        if !pwm.set_period_ns(period_ns) {
            error!("Can't set period for pwm");
            return false;
        }
        if !pwm.set_duty_ns(duty_ns) {
            error!("Can't set duty for pwm");
            return false;
        }
        if !pwm.set_enable(true) {
            error!("Can't enable pwm");
            return false;
        }
        true
    }
}