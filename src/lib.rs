#[cfg(windows)]
extern crate winutil;
extern crate regex;

#[cfg(target_os = "linux")]
use std::path::Path;
#[cfg(target_os = "linux")]
use std::fs::File;
#[cfg(target_os = "linux")]
use std::io::BufReader;
#[cfg(target_os = "linux")]
use std::io::BufRead;
#[cfg(target_os = "linux")]
use std::io::Read;
use std::process::Command;
#[cfg(target_os = "linux")]
use regex::Regex;

/// Enum listing all currently supported OS
///
/// Note: testing has not been completed with all.
#[derive(Debug, PartialEq)]
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
    Windows,
    Unknown
}

/// Contains the OS and version.
///
/// For Linux, the version number will be the version of
/// the kernel and for Windows, the version number will be
/// the number provided by `ver`. For MacOS, the version
/// number is the product version number from `sw_vers`
#[derive(Debug, PartialEq)]
pub struct OSInformation {
    pub os_type: OSType,
    pub version: String
}

/// Contains information about the filesystem
#[derive(Debug, PartialEq)]
pub struct DiskInfo {
    pub total: Option<u64>,
    pub free: Option<u64>,
    pub in_use: Option<u64>,
}

#[derive(Debug, PartialEq)]
pub struct MemInfo {
    pub total: Option<u64>,
    pub free: Option<u64>,
    pub in_use: Option<u64>
}

#[derive(Debug, PartialEq)]
pub struct CPUInfo {
    pub num: Option<usize>,
    pub model: Option<String>,
    pub mhz: Option<String>
}

/// Generates a generic OSInformation for an unknown or
/// undetectable OS.
#[cfg(target_os = "linux")]
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
#[cfg(target_os = "linux")]
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

#[cfg(target_os = "macos")]
pub fn get_hostname() -> Option<String> {
    match Command::new("hostname").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap().trim().to_owned()),
        Err(why) => panic!("{:?}", why)
    }
}

/// Gets the hostname for Windows systems.
#[cfg(target_os = "windows")]
pub fn get_hostname() -> Option<String> {
    winutil::get_computer_name()
}

/// Gets the current username for MacOS and Linux systems.
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_username() -> Option<String> {
    match Command::new("id").args(&["-u", "-n"]).output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap().trim().to_owned()),
        Err(why) => panic!("{:?}", why)
    }
}

/// Gets the current username for Windows systems.
#[cfg(target_os = "windows")]
pub fn get_username() -> Option<String> {
    winutil::get_user_name()
}

/// Generates an OSInformation for MacOS.
#[cfg(target_os = "macos")]
pub fn get_os() -> OSInformation {
    let sw_vers: String = match Command::new("sw_vers").arg("-productVersion").output() {
        Ok(sw_vers) => String::from_utf8(sw_vers.stdout).unwrap().trim().to_owned(),
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

/// Helper function for get_os_linux()
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

/// Generates an OSInformation for Windows systems.
///
/// Uses the DOS `ver` command for the version.
/// [Table for interpretation.](https://en.wikipedia.org/wiki/Ver_(command)#Version_list)
/// Currently untested.
#[cfg(target_os = "windows")]
pub fn get_os() -> OSInformation {
    let systeminfo = match Command::new("systeminfo").output() {
    	Ok(output) => String::from_utf8(output.stdout).unwrap(),
    	Err(why) => panic!("{:?}", why),
    };

    let re = Regex::new(r"OS Version:\s+\d+\.\d+\.\d+").unwrap();
    let mat = re.find(&systeminfo).unwrap();
    let ver_vec: Vec<&str> = systeminfo[mat.start()..mat.end()].split(":").collect();
    let win_ver = ver_vec[1].trim();

    OSInformation {
        os_type: OSType::Windows,
        version: win_ver.to_owned(),
    }
}

/// Generates a DiskInfo for Windows and unix systems
///
/// Runs system_information/src/cpp/disk to get total,
/// free, and in use space for the C or / filesystems.
pub fn get_disk_info(fs: &str) -> DiskInfo {
    let mut path = std::env::current_dir().unwrap();
    path.push("src");
    path.push("cpp");
    path.push("disk");

    let disk_path = path.to_str().unwrap().to_owned();

    let disk_info = match Command::new(disk_path).arg(fs).output() {
        Ok(info) => String::from_utf8(info.stdout).unwrap(),
        Err(why) => panic!(why),
    };

    let info_vec: Vec<&str> = disk_info.split("\n").collect();

    DiskInfo {
        total: Some(info_vec[0].parse().unwrap()),
        free: Some(info_vec[1].parse().unwrap()),
        in_use: Some(info_vec[2].parse().unwrap()),
    }
}

/// Makes the output from get_disk_info() human readable.
pub fn get_readable_disk_info(fs: &str) -> Vec<String> {
    let disk_info = get_disk_info(fs);

    let mut readable_info = Vec::new();

    if disk_info.total.is_some() {
        readable_info.push(readable_bytes(disk_info.total.unwrap(),  true));
    }
    if disk_info.free.is_some() {
        readable_info.push(readable_bytes(disk_info.free.unwrap(),   true));
    }
    if disk_info.in_use.is_some() {
        readable_info.push(readable_bytes(disk_info.in_use.unwrap(), true));
    }

    return readable_info
}

/// Helper function for get_readable_disk_info()
fn readable_bytes(size: u64, si: bool) -> String {
    let mut size_cpy = size;

    let byte_units;
    let unit;

    if si {
        byte_units = [" B", " kB", " MB", " GB", " TB", " PB", " EB", " ZB", " YB"];
        unit = 1000;
    }
    else {
        byte_units = [" B", " KiB", " MiB", " GiB", " TiB", " PiB", " EiB", " ZiB", " YiB"];
        unit = 1024;
    }

    let mut count = 0;
    while (size_cpy >= unit) && (count < byte_units.len() - 1) {
        count += 1;
        size_cpy = size_cpy / unit;
    }

    return size_cpy.to_string() + byte_units[count];
}

/// Gets memory information for Linux, returns MemInfo
///
/// Reads from /proc/meminfo to get info
#[cfg(target_os = "linux")]
pub fn get_mem_info() -> MemInfo {
    let path = Path::new("/proc/meminfo");
    let meminfo = match File::open(&path) {
        Ok(file) => BufReader::new(file),
        Err(why) => panic!("{:?}", why),
    };

    let mut mem_total: u64 = 0;
    let mut mem_free: u64 = 0;

    for line in meminfo.lines() {
        let l = line.unwrap();

        let l_vec: Vec<&str> = l.split_whitespace().collect();

        if l_vec[0] == "MemTotal:" {
            mem_total = l_vec[1].parse().unwrap();
        }
        else if l_vec[0] == "MemFree:" {
            mem_free = l_vec[1].parse().unwrap();
        }
    }

    MemInfo {
        total: Some(mem_total * 1024),
        free: Some(mem_free * 1024),
        in_use: Some((mem_total * 1024) - (mem_free * 1024))
    }
}

/// Gets memory information for Windows
///
/// Use `make windows` to build the relevant C++
/// files for Windows, they won't compile if you
/// aren't on Windows because they need windows.h
#[cfg(windows)]
pub fn get_mem_info() -> MemInfo {
    let mut path = std::env::current_dir().unwrap();
    path.push("src");
    path.push("cpp");
    path.push("mem-windows");

    let mem_path = path.to_str().unwrap().to_owned();

    let mem_info = match Command::new(mem_path).output() {
        Ok(info) => String::from_utf8(info.stdout).unwrap(),
        Err(why) => panic!(why),
    };

    let info_vec: Vec<&str> = mem_info.split("\n").collect();

    let mem_total = info_vec[0].trim().parse().unwrap();
    let mem_free = info_vec[1].trim().parse().unwrap();
    let mem_in_use = info_vec[2].trim().parse().unwrap();

    MemInfo {
        total: Some(mem_total),
        free: Some(mem_free),
        in_use: Some(mem_in_use)
    }
}

/// Gets memory information for MacOS
///
/// Currently not implemented
#[cfg(target_os = "macos")]
pub fn get_mem_info() -> MemInfo {

    let mem_info = match Command::new("sysctl").arg("hw.memsize").output() {
        Ok(output) => String::from_utf8(output.stdout).unwrap(),
        Err(why) => panic!(why)
    };

    let mem_vec: Vec<&str> = mem_info.split_whitespace().collect();

    let mem_total = mem_vec[1].parse().unwrap();

    MemInfo {
        total: Some(mem_total),
        free: None,
        in_use: None
    }
}

/// Converts the given MemInfo into human
/// readable strings
///
/// It currently uses the 1024 non-SI based
/// conversion for RAM, I'm not certain that
/// is the correct way to do it
pub fn get_readable_mem_info() -> Vec<String> {
    let mem_info = get_mem_info();

    let mut readable_info = Vec::new();

    if mem_info.total.is_some() {
        readable_info.push(readable_bytes(mem_info.total.unwrap(), true));
    } else { readable_info.push("".to_owned()); }
    if mem_info.free.is_some() {
        readable_info.push(readable_bytes(mem_info.free.unwrap(), true));
    } else { readable_info.push("".to_owned()); }
    if mem_info.in_use.is_some() {
        readable_info.push(readable_bytes(mem_info.in_use.unwrap(), true));
    } else { readable_info.push("".to_owned()); }

    return readable_info
}

#[cfg(target_os = "linux")]
pub fn get_cpu_info() -> CPUInfo {
    let path = Path::new("/proc/cpuinfo");

    let mut cpu_file = match File::open(&path) {
        Ok(file) => BufReader::new(file),
        Err(why) => panic!(why)
    };

    let mut cpuinfo = String::new();
    cpu_file.read_to_string(&mut cpuinfo).ok();

    CPUInfo {
        num: get_cpu_num_linux(&cpuinfo),
        model: get_cpu_model_linux(&cpuinfo),
        mhz: get_cpu_mhz_linux(&cpuinfo)
    }

}

#[cfg(target_os = "linux")]
fn get_cpu_num_linux(cpuinfo: &str) -> Option<usize> {
    let re = Regex::new(r"processor\s+:\s\d").unwrap();

    let iter = re.captures_iter(&cpuinfo);

    return Some(iter.count());
}

#[cfg(target_os = "linux")]
fn get_cpu_model_linux(cpuinfo: &str) -> Option<String> {
    let re = Regex::new(r"model\sname\s+:.+").unwrap();

    let mat = re.find(&cpuinfo).unwrap();

    let model_vec: Vec<&str> = cpuinfo[mat.start()..mat.end()].split(":").collect();

    let cpu_model = model_vec[1].trim();

    return Some(cpu_model.to_owned());

}

#[cfg(target_os = "linux")]
fn get_cpu_mhz_linux(cpuinfo: &str) -> Option<String> {
    let re = Regex::new(r"cpu\sMHz\s+:.+").unwrap();

    let mat = re.find(&cpuinfo).unwrap();

    let mhz_vec: Vec<&str> = cpuinfo[mat.start()..mat.end()].split(":").collect();

    let cpu_mhz = mhz_vec[1].trim();

    return Some(cpu_mhz.to_owned());

}

#[cfg(target_os = "macos")]
pub fn get_cpu_info() -> CPUInfo {
    CPUInfo {
        num: get_cpu_num_macos(),
        model: get_cpu_model_macos(),
        mhz: get_cpu_mhz_macos()
    }
}

#[cfg(target_os = "macos")]
fn get_cpu_num_macos() -> Option<usize> {
    let num = match Command::new("sysctl").arg("hw.ncpu").output() {
        Ok(output) => String::from_utf8(output.stdout).unwrap(),
        Err(why) => panic!(why)
    };

    let num_vec: Vec<&str> = num.split(":").collect();
    let cpu_num = num_vec[1].trim().parse().unwrap();
    return Some(cpu_num);
}

#[cfg(target_os = "macos")]
fn get_cpu_model_macos() -> Option<String> {
    let model = match Command::new("sysctl").arg("machdep.cpu.brand_string").output() {
        Ok(output) => String::from_utf8(output.stdout).unwrap(),
        Err(why) => panic!(why)
    };

    let model_vec: Vec<&str> = model.split(":").collect();
    let cpu_model = model_vec[1].trim().to_owned();
    return Some(cpu_model);
}

#[cfg(target_os = "macos")]
fn get_cpu_mhz_macos() -> Option<String> {
    let mhz = match Command::new("sysctl").arg("hw.cpufrequency").output() {
        Ok(output) => String::from_utf8(output.stdout).unwrap(),
        Err(why) => panic!(why)
    };

    let mhz_vec: Vec<&str> = mhz.split(":").collect();
    let mhz_string = mhz_vec[1].trim().to_owned();
    let cpu_mhz: u64 = mhz_string.parse().unwrap();
    return Some((cpu_mhz / 1000000).to_string());
}

#[cfg(target_os = "windows")]
pub fn get_cpu_info() -> CPUInfo {
    CPUInfo {
        num: get_cpu_num_windows(),
        model: get_cpu_model_windows(),
        mhz: get_cpu_mhz_windows()
    }
}

#[cfg(target_os = "windows")]
fn get_cpu_num_windows() -> Option<usize> {
    return Some(1);
}

#[cfg(target_os = "windows")]
fn get_cpu_model_windows() -> Option<String> {
    return Some("Nada".to_owned());
}

#[cfg(target_os = "windows")]
fn get_cpu_mhz_windows() -> Option<String> {
    return Some("nada".to_owned());
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
        let os_type = os.os_type; let version = os.version;
        assert_eq!(os_type, OSType::Unknown);
        assert_eq!(version, "0.0.0".to_owned());
    }

    #[test]
    fn disk() {
        let disk = get_disk_info("/");

        assert_eq!(disk, DiskInfo {
            total: Some(0),
            free: Some(0),
            in_use: Some(0),
        });
    }

    #[test]
    fn readable_disk() {
        let readable_info = get_readable_disk_info("/");

        assert_eq!(readable_info, vec!["".to_owned(), "".to_owned(), "".to_owned()]);
    }

    #[test]
    fn mem() {
        let mem = get_mem_info();

        assert_eq!(mem, MemInfo {
            total: Some(0),
            free: Some(0),
            in_use: Some(0)
        });
    }

    #[test]
    fn readable_mem() {
        let readable_mem = get_readable_mem_info();

        assert_eq!(readable_mem, vec!["".to_owned(), "".to_owned(), "".to_owned()]);
    }

    #[test]
    fn cpu_info() {
        let cpu_info = get_cpu_info();

        assert_eq!(cpu_info, CPUInfo {
            num: Some(0),
            model: Some("".to_owned()),
            mhz: Some("".to_owned())
        })
    }

}
