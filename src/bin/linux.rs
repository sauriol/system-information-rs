extern crate regex;

use regex::Regex;
use std::process::Command;

struct Linux_Info {
    distro: String,
    version: String
}

pub fn get_username() -> Option<String> {
    match Command::new("id").args(&["-u", "-n"]).output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}

pub fn get_hostname() -> Option<String> {
    match Command::new("cat").arg("/proc/sys/kernel/hostname").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None
    }
}
