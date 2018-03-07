extern crate regex;

use std::process::Command;
use regex::Regex;

struct OsxInfo {
    product_name: Option<String>,
    product_version: Option<String>,
    build_version: Option<String>,
}

pub fn is_osx() -> bool {
    match Command::new("sw_vers").output() {
        Ok(output) => output.status.success(),
        Err(_) => false
    }
}

pub fn get_sw_vers() -> Option<OsxInfo> {
    let sw_vers = match Command::new("sw_vers").output() {
        Ok(sw_vers) => sw_vers,
        Err(_) => return None
    };

    let stdout = String::from_utf8(sw_vers.stdout).unwrap();
    parse(stdout)
}

fn parse(sw_vers_string: String) -> Option<OsxInfo> {
    let product_name_regex = Regex::new(r"ProductName:\s+([\w\s]+)\n").unwrap();
    let product_version_regex = Regex::new(r"ProductVersion:\s(\d+[.]\d+[.]\d+)").unwrap();
    let build_number_regex = Regex::new(r"BuildVersion:\s+(\w+)").unwrap();

    Some(OsxInfo {
        product_name: extract_value(&sw_vers_string, product_name_regex),
        product_version: extract_value(&sw_vers_string, product_version_regex),
        build_version: extract_value(&sw_vers_string, build_number_regex)
    })
}

fn extract_value(stdout: &String, re: Regex) -> Option<String> {
    let caps = re.captures_iter(&stdout);

    match caps.next() {
        Some(data) => {
            match data.get(1) {
                Some(occ) => {
                    Some(occ.as_str().to_owned())
                },
                None => None
            }
        },
        None => None
    }
}

pub fn get_username() -> Option<String> {
    match Command::new("id").args(&["-u", "-n"]).output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None
    }
}

/// Possibly change to read from /proc/sys/kernel/hostname
pub fn get_hostname() -> Option<String> {
    match Command::new("hostname").output() {
        Err(_) => None,
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
    }
}
