extern crate regex;

use regex::Regex;
use std::process::Command;
use std::path::Path;

struct DistroInfo {
    distro: Option<String>,
    version: Option<String>,
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

pub fn get_distro_info() -> Option<DistroInfo> {

}

fn get_distro() -> Option<String> {
    if Path::new("/etc/os-release").exists() {
        return parse_os_release()
    }
    else if Path::new("/etc/arch-release").exists() {
        return Some("Arch Linux".to_owned())
    }
    else if Path::new("/etc/gentoo-release").exists() {
        return Some("Gentoo Linux".to_owned())
    }
    else if Path::new("/etc/fedora-release").exists() {
        return Some("Fedora Linux".to_owned())
    }
    else if Path::new("/etc/redhat-release").exists() {
        return Some("Red Hat Enterprise Linux".to_owned())
    }
    else if Path::new("/etc/debian_version").exists() {
        return Some("Debian GNU/Linux".to_owned())
    }
    else {
        return None
    }
}

/// Parses the /etc/os-release file and returns the PRETTY_NAME
fn parse_os_release() -> Option<String> {

}
