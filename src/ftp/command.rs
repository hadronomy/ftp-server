use miette::*;

use crate::ftp::StatusCode;

/// The FTP commands
///
/// See [RFC 959](https://tools.ietf.org/html/rfc959)
pub enum Command<'a> {
    /// **USER** - Specify user for authentication
    User(&'a str),

    /// **PASS** - Specify password for authentication
    Pass(&'a str),

    /// **CWD** - Change working directory
    Cwd(&'a str),

    /// **CDUP** - Change to parent directory
    Cdup,

    /// **QUIT** - Disconnection
    Quit,

    /// **PORT** - Data port
    Port,

    /// **PASV** - Passive mode
    Pasv,

    /// **TYPE** - Representation type
    Type,

    /// **MODE** - Transfer mode
    Mode,

    /// **STRU** - File structure
    Stru,

    /// **RETR** - Retrieve a copy of the file
    Retr,

    /// **STOR** - Store a file
    Stor,

    /// **NOOP** - Do nothing
    Noop,

    /// **SYST** - Get operating system type
    Syst,

    /// **STAT** - Get data connection status
    Stat,

    /// **HELP** - Help
    Help,

    /// **DELE** - Delete file
    Dele,

    /// **RMD** - Remove directory
    Rmd,

    /// **MKD** - Make directory
    Mkd,

    /// **PWD** - Print working directory
    Pwd,

    /// **LIST** - List files
    List,

    /// **NLST** - Name list of files
    Nlst,

    /// **SITE** -
    Site,

    /// **Unknown** - Unknown command
    Unknown,
}

impl<'a> Command<'a> {
    pub fn run(&self) -> Result<StatusCode> {
        match self {
            Command::User(_) => todo!(),
            Command::Pass(_) => todo!(),
            Command::Cwd(_) => todo!(),
            Command::Cdup => todo!(),
            Command::Quit => todo!(),
            Command::Port => todo!(),
            Command::Pasv => todo!(),
            Command::Type => todo!(),
            Command::Mode => todo!(),
            Command::Stru => todo!(),
            Command::Retr => todo!(),
            Command::Stor => todo!(),
            Command::Noop => todo!(),
            Command::Syst => todo!(),
            Command::Stat => todo!(),
            Command::Help => todo!(),
            Command::Dele => todo!(),
            Command::Rmd => todo!(),
            Command::Mkd => todo!(),
            Command::Pwd => todo!(),
            Command::List => todo!(),
            Command::Nlst => todo!(),
            Command::Site => todo!(),
            Command::Unknown => todo!(),
        }
    }
}
