use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct System {
    name: &'static str,
}

impl From<SystemType> for System {
    fn from(system: SystemType) -> Self {
        match system {
            SystemType::Unix => Self {
                name: "UNIX Type: L8",
            },
            SystemType::Windows => Self { name: "Windows_NT" },
            SystemType::MacOS => Self {
                name: "MACOS Type: L8",
            },
            SystemType::Linux => Self {
                name: "UNIX Type: L8",
            },
            SystemType::Android => Self {
                name: "Android Type: L8",
            },
            SystemType::Ios => Self {
                name: "IOS Type: L8",
            },
            SystemType::Unknown => Self {
                name: "Unknown Type: L8",
            },
        }
    }
}

impl Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub enum SystemType {
    Unix,
    Windows,
    MacOS,
    Linux,
    Android,
    Ios,
    Unknown,
}

impl SystemType {
    pub fn from_os() -> Self {
        match std::env::consts::OS {
            "linux" => Self::Linux,
            "macos" => Self::MacOS,
            "windows" => Self::Windows,
            "android" => Self::Android,
            "ios" => Self::Ios,
            _ => Self::Unknown,
        }
    }
}

impl ToString for SystemType {
    fn to_string(&self) -> String {
        System::from(self.clone()).to_string()
    }
}
