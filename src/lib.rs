#[cfg(target_os = "windows")]
extern crate winutil;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::process::Command;

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

/// Holds info about OS type and version
pub struct OSInformation {
    os_type: OSType,
    version: String
}

fn unknown_os() -> OSInformation {
    OSInformation {
        os_type: OSType::Unknown,
        version: "0.0.0".to_owned()
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn get_hostname() -> Option<String> {
    let path = Path::new("/proc/sys/kernel/hostname");

    let mut hostname_file = match File::open(&path) {
        Ok(file) => BufReader::new(file),
        Err(why) => panic!("{:?}", why)
    };

    let mut hostname = String::new();
    hostname_file.read_to_string(&mut hostname);

    Some(hostname.trim().to_owned())
}

#[cfg(windows)]
pub fn get_hostname() -> Option<String> {
    winutil::get_computer_name()
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn get_username() -> Option<String> {
    match Command::new("id").args(&["-u", "-n"]).output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap().trim().to_owned()),
        Err(why) => panic!("{:?}", why)
    }
}

#[cfg(windows)]
pub fn get_username() -> Option<String> {
    winutil::get_user_name()
}

#[cfg(target_os = "macos")]
pub fn get_os() -> OSInformation {
    let sw_vers = match Command::new("sw_vers").output() {
        Ok(sw_vers) => sw_vers,
        Err(why) => panic!("{:?}", why)
    };

    /// Need to implement some getting of the product_version from sw_vers

    OSInformation {
        os_type: OSType::OSX,
        version: "Not yet supported".to_owned()
    }
}

#[cfg(target_os = "linux")]
pub fn get_os() -> OSInformation {
    if Path::new("/etc/arch-release").exists() {
        OSInformation {
            os_type: OSType::Arch,
            version: "Not yet supported".to_owned()
        }
    }
    else if Path::new("/etc/debian_version").exists() {
        OSInformation {
            os_type: OSType::Debian,
            version: "Not yet supported".to_owned()
        }
    }
    else if Path::new("/etc/fedora-release").exists() {
        OSInformation {
            os_type: OSType::Fedora,
            version: "Not yet supported".to_owned()
        }
    }
    else if Path::new("/etc/gentoo-release").exists() {
        OSInformation {
            os_type: OSType::Gentoo,
            version: "Not yet supported".to_owned()
        }
    }
    else if Path::new("/etc/redhat-release").exists() {
        OSInformation {
            os_type: OSType::Redhat,
            version: "Not yet supported".to_owned()
        }
    }
    else if Path::new("/etc/os-release").exists() {
        parse_os_release()
    }
    else {
        unknown_os()
    }
}

#[cfg(target_os = "linux")]
fn parse_os_release() -> OSInformation {

}

/// I don't know how to get just the version number for this
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

}
