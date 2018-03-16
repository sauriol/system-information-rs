#[cfg(target_os = "windows")]
extern crate winutil;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;
use std::process::Command;

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

// Holds info about OS type and version
pub struct OSInformation {
    pub os_type: OSType,
    pub version: String
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
    hostname_file.read_to_string(&mut hostname).ok();

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

// Needs to be tested
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
    else if Path::new("/etc/SuSE-release").exists() {
        OSInformation {
            os_type: OSType::OpenSUSE,
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
    let path = Path::new("/etc/os-release");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!(why),
    };
    let file_read = BufReader::new(&file);

    let mut os: OSType = OSType::Unknown;

    for line in file_read.lines() {
        let l = line.unwrap();

        let l_vec: Vec<&str> = l.split('"').collect();

        if l_vec[0] == "NAME=".to_owned() {
            os = match_os(l_vec[1]);
            break;
        }
    }

    OSInformation {
        os_type: os,
        version: "Not yet supported".to_owned()
    }
}

// all of these need to be tested and some might not work
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

// I don't know how to get just the version number for this
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
        assert_eq!(version, "Not yet supported".to_owned());
    }

}
