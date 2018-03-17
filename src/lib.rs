#[cfg(target_os = "windows")]
extern crate winutil;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;
use std::process::Command;

/// Enum listing all currently supported OS
///
/// Note: testing has not been completed with all.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum OSType {
    Arch,
    CentOS,
    Debian,
    Fedora,
    Gentoo,
    OpenSUSE,
    OSX,
    Redhat,
    Ubuntu,
    Unknown
}

/// Contains the OS and version.
///
/// For Linux, the version number will be the version of
/// the kernel and for Windows, the version number will be
/// the number provided by `ver`. For MacOS, the version
/// number is the product version number from `sw_vers`
pub struct OSInformation {
    pub os_type: OSType,
    pub version: String
}

/// Generates a generic OSInformation for an unknown or 
/// undetectable OS.
fn unknown_os() -> OSInformation {
    OSInformation {
        os_type: OSType::Unknown,
        version: "0.0.0".to_owned()
    }
}

/// Gets the hostname on MacOS and Linux systems.
///
/// Reads from /proc/sys/kernel/hostname to find the
/// current hostname. Currently untested on MacOS.
#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn get_hostname() -> Option<String> {
    let path = Path::new("/proc/sys/kernel/hostname");

    let mut hostname_file = match File::open(&path) {
        Ok(file) => BufReader::new(file),
        Err(why) => panic!("{:?}", why)
    };

    let mut hostname = String::new();
    hostname_file.read_to_string(&mut hostname).ok();

    Some(hostname.trim().to_owned())
}

/// Gets the hostname for Windows systems.
#[cfg(windows)]
pub fn get_hostname() -> Option<String> {
    winutil::get_computer_name()
}

/// Gets the current username for MacOS and Linux systems.
///
/// Currently untested on MacOS and I'm not certain that 
/// the `id` command exists on all systems.
#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn get_username() -> Option<String> {
    match Command::new("id").args(&["-u", "-n"]).output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap().trim().to_owned()),
        Err(why) => panic!("{:?}", why)
    }
}

/// Gets the current username for Windows systems.
#[cfg(windows)]
pub fn get_username() -> Option<String> {
    winutil::get_user_name()
}

/// Generates an OSInformation for MacOS.
///
/// Currently untested.
#[cfg(target_os = "macos")]
pub fn get_os() -> OSInformation {
    let sw_vers: String = match Command::new("sw_vers").arg("-productVersion").output() {
        Ok(sw_vers) => sw_vers.trim().to_owned(),
        Err(why) => panic!("{:?}", why)
    };

    OSInformation {
        os_type: OSType::OSX,
        version: sw_vers
    }
}

/// Generates an OSInformation for Linux systems.
///
/// Currently only tested on Arch Linux.
#[cfg(target_os = "linux")]
pub fn get_os() -> OSInformation {
    if Path::new("/etc/arch-release").exists() {
        OSInformation {
            os_type: OSType::Arch,
            version: get_ver()
        }
    }
    else if Path::new("/etc/debian_version").exists() {
        OSInformation {
            os_type: OSType::Debian,
            version: get_ver()
        }
    }
    else if Path::new("/etc/fedora-release").exists() {
        OSInformation {
            os_type: OSType::Fedora,
            version: get_ver()
        }
    }
    else if Path::new("/etc/gentoo-release").exists() {
        OSInformation {
            os_type: OSType::Gentoo,
            version: get_ver()
        }
    }
    else if Path::new("/etc/SuSE-release").exists() {
        OSInformation {
            os_type: OSType::OpenSUSE,
            version: get_ver()
        }
    }
    else if Path::new("/etc/redhat-release").exists() {
        OSInformation {
            os_type: OSType::Redhat,
            version: get_ver()
        }
    }
    else if Path::new("/etc/os-release").exists() {
        parse_os_release()
    }
    else {
        unknown_os()
    }
}

/// Helper function for get_os()
///
/// Parses /etc/os-release to get the OS name.
/// Currently untested.
#[cfg(target_os = "linux")]
fn parse_os_release() -> OSInformation {
    let path = Path::new("/etc/os-release");
    let file = match File::open(&path) {
        Ok(file) => BufReader::new(file),
        Err(why) => panic!("{:?}", why),
    };

    let mut os: OSType = OSType::Unknown;

    for line in file.lines() {
        let l = line.unwrap();

        let l_vec: Vec<&str> = l.split('"').collect();

        if l_vec[0] == "NAME=".to_owned() {
            os = match_os(l_vec[1]);
            break;
        }
    }

    OSInformation {
        os_type: os,
        version: get_ver()
    }
}

/// Helper function for parse_os_release().
///
/// Matches the NAME from /etc/os-release to the
/// corresponding distro.
#[cfg(target_os = "linux")]
fn match_os(os_str: &str) -> OSType {
    if os_str == "Arch Linux".to_owned() {
        return OSType::Arch;
    }
    else if os_str == "CentOS Linux".to_owned() {
        return OSType::CentOS;
    }
    else if os_str == "Debian GNU/Linux".to_owned() {
        return OSType::Debian;
    }
    else if os_str == "Fedora".to_owned() {
        return OSType::Fedora;
    }
    else if os_str == "Red Hat Enterprise Linux Server".to_owned() {
        return OSType::Redhat;
    }
    else {
        return OSType::Unknown
    }

}

/// Gets kernel version of Linux systems.
///
/// Reads from /proc/sys/kernel/osrelease to find the
/// current kernel version.
#[cfg(target_os = "linux")]
fn get_ver() -> String {
    let path = Path::new("/proc/sys/kernel/osrelease");
    let mut osrelease = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("{:?}", why),
    };

    let mut ver = String::new();

    osrelease.read_to_string(&mut ver).ok();

    ver.trim().to_owned()
}

/// Gets version of Windows systems.
///
/// Uses the DOS `ver` command.
/// [Table for interpretation](https://en.wikipedia.org/wiki/Ver_(command)#Version_list)
#[cfg(target_os = "windows")]
pub fn get_os() -> Option<OSInformation> {
    let win_ver = match Command::new("ver").output() {
        Ok(ver) => ver,
        Err(why) => panic!("{:?}", why)
    };

    OSInformation {
        os_type: OSType::Windows,
        version: "Not yet supported".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hostname() {
        let hostname = get_hostname().unwrap();
        assert_eq!(hostname, "[hostname]");
    }

    #[test]
    fn username() {
        let username = get_username().unwrap();
        assert_eq!(username, "[username]");
    }

    #[test]
    fn os() {
        let os = get_os();
        let os_type = os.os_type;
        let version = os.version;

        assert_eq!(os_type, OSType::Unknown);
        assert_eq!(version, "0.0.0".to_owned());
    }

}
