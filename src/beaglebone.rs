use crate::pin::*;
use std::path::Path;
use std::fs;
use std::{thread, time};

impl Pwm {
    pub fn is_enable(&self) -> bool {
        let enabled_path = format!("/sys/class/pwm/pwm-{}:{}/enable", self.sysfs, self.index);
        let enabled = fs::read_to_string(&enabled_path).unwrap_or(String::from("0"));
        let enabled = enabled.trim();
        enabled == "1"
    }

    pub fn set_enable(&self, enable: bool) -> bool {
        let enabled_path = format!("/sys/class/pwm/pwm-{}:{}/enable", self.sysfs, self.index);
        match enable {
            true => fs::write(&enabled_path, String::from("1")).is_ok(),
            _ => fs::write(&enabled_path, String::from("0")).is_ok()
        }
    }

    pub fn get_duty_ns(&self) -> u32 {
        let duty_ns_path = format!("/sys/class/pwm/pwm-{}:{}/duty_cycle", self.sysfs, self.index);
        let duty_ns = fs::read_to_string(&duty_ns_path).unwrap_or(String::from("0"));
        let duty_ns = duty_ns.trim();
        duty_ns.parse::<u32>().unwrap_or(0)
    }

    pub fn get_period_ns(&self) -> u32 {
        let period_ns_path = format!("/sys/class/pwm/pwm-{}:{}/period", self.sysfs, self.index);
        let period_ns = fs::read_to_string(&period_ns_path).unwrap_or(String::from("0"));
        let period_ns = period_ns.trim();
        period_ns.parse().unwrap_or(0)
    }

    pub fn set_duty_ns(&mut self, duty_ns: u32) -> bool {
        let duty_ns_path = format!("/sys/class/pwm/pwm-{}:{}/duty_cycle", self.sysfs, self.index);
        let ok = fs::write(&duty_ns_path, duty_ns.to_string()).is_ok();
        info!("Write {} in {} - {}", duty_ns, duty_ns_path, ok);
        ok
    }

    pub fn set_period_ns(&mut self, period_ns: u32) -> bool {
        let period_ns_path = format!("/sys/class/pwm/pwm-{}:{}/period", self.sysfs, self.index);
        let new_duty_ns = (period_ns as f32 / self.get_period_ns() as f32) * self.get_duty_ns() as f32;
        if period_ns < self.get_duty_ns() {
            // Going to a shorter period, update duty_cycle first to avoid any error
            if !self.set_duty_ns(new_duty_ns as u32) {
                error!("Can't change duty_cycle");
                return false;
            }
        }
        // Update freq
        info!("Write {} in {}", period_ns, period_ns_path);
        if !fs::write(&period_ns_path, period_ns.to_string()).is_ok() {
            error!("Can't change period {}: {}. Maybe 2 pwm uses the same pwmchip. Please see log in /var/log/syslog.", period_ns_path, period_ns);
            return false;
        }
        if period_ns > self.get_period_ns() {
            // Update duty cycle to keep level
            if !self.set_duty_ns(new_duty_ns as u32) {
                error!("Can't change duty_cycle");
                return false;
            }
        }
        true
    }

    fn pwm_setup(&self) -> bool {
        // For BBB SEEED, uboot is enabled (/bin/grep -c bone_capemgr.uboot_capemgr_enabled=1 /proc/cmdline)
        // So, there is no need to initialize pwm mode.
        // First, set the PIN in pwm mode
        let state_path = format!("/sys/devices/platform/ocp/ocp:{}_pinmux/state", self.key);
        fs::write(&state_path, "pwm").expect("Unable to write file");
        // Second, init the PIN state
        let pwm_path = format!("/sys/devices/platform/ocp/{}.epwmss/{}.pwm/pwm/pwmchip{}",
            self.chip, self.addr, self.sysfs);
        let exported_path = format!("{}/pwm-{}:{}", pwm_path, self.sysfs, self.index);
        if !Path::new(&exported_path).exists() {
            // Export pin
            let export_path = format!("{}/export", pwm_path);
            fs::write(&export_path, self.index.to_string()).expect("Unable to write file");
            thread::sleep(time::Duration::from_millis(100));
        }
        true
    }

    pub fn start_pwm(&mut self, duty_ns: u32, period_ns: u32) -> bool {
        if !self.pwm_setup() {
            error!("Can't setup pwm");
            return false;
        }
        if !self.set_period_ns(period_ns) {
            error!("Can't set period for pwm");
            return false;
        }
        if !self.set_duty_ns(duty_ns) {
            error!("Can't set duty for pwm");
            return false;
        }
        if !self.set_enable(true) {
            error!("Can't enable pwm");
            return false;
        }
        true
    }
}