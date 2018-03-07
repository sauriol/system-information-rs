

use std::process::Command;
use std::path::Path;

enum OSType {
    Arch,
    CentOS,
    Debian,
    Fedora,
    OpenSUSE,
    OSX,
    Redhat,
    Ubuntu,
    Unknown
}

/// Holds info about OS type and version
struct OSInformation {
    os_type: self::OSType,
    version: String
}

fn unknown_os() -> OSInformation {
    OSInformation {
        os_type: OSType::Unknown,
        version: "0.0.0".to_owned()
    }
}

