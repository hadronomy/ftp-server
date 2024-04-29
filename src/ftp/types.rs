pub struct System {
    name: &'static str,
    features: &'static [&'static str],
}

impl From<SystemType> for System {
    fn from(system: SystemType) -> Self {
        match system {
            SystemType::Unix => Self {
                name: "UNIX Type: L8",
                features: &["SIZE", "MDTM", "UTF8"],
            },
            SystemType::Windows => Self {
                name: "Windows_NT",
                features: &["SIZE", "MDTM", "UTF8"],
            },
            SystemType::MacOS => Self {
                name: "MACOS Type: L8",
                features: &["SIZE", "MDTM", "UTF8"],
            },
            SystemType::Linux => Self {
                name: "LINUX Type: L8",
                features: &["SIZE", "MDTM", "UTF8"],
            },
            SystemType::Android => Self {
                name: "Android Type: L8",
                features: &["SIZE", "MDTM", "UTF8"],
            },
            SystemType::IOS => Self {
                name: "IOS Type: L8",
                features: &["SIZE", "MDTM", "UTF8"],
            },
            SystemType::Unknown => Self {
                name: "Unknown Type: L8",
                features: &["SIZE", "MDTM", "UTF8"],
            },
        }
    }
}

impl ToString for System {
    fn to_string(&self) -> String {
        let features = self.features.join("\n");
        format!("{}\n{}", self.name, features)
    }
}

#[derive(Debug, Clone)]
pub enum SystemType {
    Unix,
    Windows,
    MacOS,
    Linux,
    Android,
    IOS,
    Unknown,
}

impl SystemType {
    pub fn from_os() -> Self {
        match std::env::consts::OS {
            "linux" => Self::Linux,
            "macos" => Self::MacOS,
            "windows" => Self::Windows,
            "android" => Self::Android,
            "ios" => Self::IOS,
            _ => Self::Unknown,
        }
    }
}

impl ToString for SystemType {
    fn to_string(&self) -> String {
        System::from(self.clone()).to_string()
    }
}
