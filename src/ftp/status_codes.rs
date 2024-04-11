use std::{net::SocketAddr, num::NonZeroU16};

use tokio::net::{TcpListener, TcpStream};

use miette::*;
use thiserror::*;

#[derive(Debug, Clone)]
pub enum StatusCode {
    /// **110** - Restart marker reply.
    ///
    /// In this case, the text is exact and not left to the
    /// particular implementation; it must read:
    ///   MARK yyyy = mmmm
    ///
    /// Where yyyy is User-process data stream marker, and mmmm
    /// server's equivalent marker (note the spaces between markers and "=").
    RestartMarkerReply,

    /// **120** - Service ready in **nnn** minutes.
    ServiceReadyIn,

    /// **125** - Data connection already open; transfer starting.
    DataOpenTransfer,

    /// **150** - File status okay; about to open data connection.
    FileStatusOk,

    /// **200** - Ok
    Ok,

    /// **202** - Command not implemented, superfluous at this site.
    SuperfluousCmdNotImplemented,

    /// **211** - System status, or system help reply.
    SystemStatus,

    /// **212** - Directory status.
    DirectoryStatus,

    /// **213** - File status.
    FileStatus,

    /// **214** - Help message.
    HelpMsg { message: String },

    /// **215** - NAME system type.
    /// Where NAME is an official system name from the list in the Assigned Numbers document.
    SystemType,

    /// **220** - Service ready for new user.
    ServiceReadyUser,

    /// **225** - Data connection open; no transfer in progress.
    DataOpenNoTransfer,

    /// **226** - Closing data connection.
    ClosingDataConn,

    /// **227** - Entering Passive Mode (h1,h2,h3,h4,p1,p2).
    EnteringPassiveMode { port_high: u16, port_low: u16 },

    /// **230** - User logged in, proceed.
    UserLoggedIn,

    /// **250** - Requested file action okay, completed.
    FileActionOk,

    /// **257** - "PATHNAME" created.
    PathCreated,

    /// **331** - User name okay, need password.
    UsernameOk,

    /// **332** - Need account for login.
    NeedLoginAccount,

    /// **350** - Requested file action pending further information.
    FileActionPending,

    /// **421** - Service not available, closing control connection.
    Unnavaidable,

    /// **425** - Can't open data connection.
    CantOpenDataConn,

    /// **426** - Connection closed; transfer aborted.
    TransferAborted,

    /// **450** - Requested file action not taken.
    FileActionNotTaken,

    /// **451** - Requested action aborted: local error in processing.
    ActionAbortedLocal,

    /// **452** - Requested action not taken.
    InsufficientStorage,

    /// **500** - Syntax error, command unrecognized.
    SyntaxError,

    /// **502** - Command not implemented.
    CmdNotImplemented,

    /// **503** - Bad sequence of commands.
    CmdBadSequence,

    /// **504** - Command not implemented for that parameter.
    CmdNotImplementedParam,

    /// **530** - Not logged in.
    UserNotLoggedIn,

    /// **532** - Need account for storing files.
    NeedAccountForStore,

    /// **550** - Requested action not taken.
    ActionNotTaken,

    /// **551** - Requested action aborted: page type unknown.
    ActionAbortedPageTypeUnknown,

    /// **552** - Requested file action aborted.
    ExceededStorageAllocation,

    /// **553** - File name not allowed.
    FilenameNotAllowed,
}

impl StatusCode {
    pub fn code(&self) -> u16 {
        match self {
            StatusCode::RestartMarkerReply => 110,
            StatusCode::ServiceReadyIn => 120,
            StatusCode::DataOpenTransfer => 125,
            StatusCode::FileStatusOk => 150,
            StatusCode::Ok => 200,
            StatusCode::SuperfluousCmdNotImplemented => 202,
            StatusCode::SystemStatus => 211,
            StatusCode::DirectoryStatus => 212,
            StatusCode::FileStatus => 213,
            StatusCode::HelpMsg { message: _ } => 214,
            StatusCode::SystemType => 215,
            StatusCode::ServiceReadyUser => 220,
            StatusCode::DataOpenNoTransfer => 225,
            StatusCode::ClosingDataConn => 226,
            StatusCode::EnteringPassiveMode {
                port_high: _,
                port_low: _,
            } => 227,
            StatusCode::UserLoggedIn => 230,
            StatusCode::FileActionOk => 250,
            StatusCode::PathCreated => 257,
            StatusCode::UsernameOk => 331,
            StatusCode::NeedLoginAccount => 332,
            StatusCode::FileActionPending => 350,
            StatusCode::Unnavaidable => 421,
            StatusCode::CantOpenDataConn => 425,
            StatusCode::TransferAborted => 426,
            StatusCode::FileActionNotTaken => 450,
            StatusCode::ActionAbortedLocal => 451,
            StatusCode::InsufficientStorage => 452,
            StatusCode::SyntaxError => 500,
            StatusCode::CmdNotImplemented => 502,
            StatusCode::CmdBadSequence => 503,
            StatusCode::CmdNotImplementedParam => 504,
            StatusCode::UserNotLoggedIn => 530,
            StatusCode::NeedAccountForStore => 532,
            StatusCode::ActionNotTaken => 550,
            StatusCode::ActionAbortedPageTypeUnknown => 551,
            StatusCode::ExceededStorageAllocation => 552,
            StatusCode::FilenameNotAllowed => 553,
        }
    }
}

impl From<StatusCode> for u16 {
    fn from(val: StatusCode) -> Self {
        val.code()
    }
}

impl StatusCode {
    /// Convert the status code to a byte array
    /// ## Usage
    /// ```
    /// use ftp::StatusCode;
    ///
    /// let status_code = StatusCode::Ok;
    /// let byte_array = status_code.to_byte_array();
    /// ```
    pub fn to_byte_array(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl ToString for StatusCode {
    fn to_string(&self) -> String {
        match self {
            StatusCode::RestartMarkerReply => format!("{} Restart marker reply\n", self.code()),
            StatusCode::ServiceReadyIn => todo!(),
            StatusCode::DataOpenTransfer => format!(
                "{} Data connection already open; transfer starting\n",
                self.code()
            ),
            StatusCode::FileStatusOk => todo!(),
            StatusCode::Ok => format!("{} Ok", self.code()),
            StatusCode::SuperfluousCmdNotImplemented => todo!(),
            StatusCode::SystemStatus => todo!(),
            StatusCode::DirectoryStatus => todo!(),
            StatusCode::FileStatus => todo!(),
            StatusCode::HelpMsg { message } => format!("{} {}\n", self.code(), message),
            StatusCode::SystemType => todo!(),
            StatusCode::ServiceReadyUser => format!("{} Service ready for new user\n", self.code()),
            StatusCode::DataOpenNoTransfer => format!("{} Data connection open\n", self.code()),
            StatusCode::ClosingDataConn => format!("{} Closing data connection\n", self.code()),
            StatusCode::EnteringPassiveMode {
                port_high,
                port_low,
            } => format!(
                "{} Entering Passive Mode (127, 0, 0, 1, {port_high}, {port_low})\n",
                self.code()
            ),
            StatusCode::UserLoggedIn => todo!(),
            StatusCode::FileActionOk => {
                format!("{} Requested file action okay, completed\n", self.code())
            }
            StatusCode::PathCreated => todo!(),
            StatusCode::UsernameOk => todo!(),
            StatusCode::NeedLoginAccount => todo!(),
            StatusCode::FileActionPending => todo!(),
            StatusCode::Unnavaidable => todo!(),
            StatusCode::CantOpenDataConn => todo!(),
            StatusCode::TransferAborted => todo!(),
            StatusCode::FileActionNotTaken => todo!(),
            StatusCode::ActionAbortedLocal => todo!(),
            StatusCode::InsufficientStorage => todo!(),
            StatusCode::SyntaxError => todo!(),
            StatusCode::CmdNotImplemented => format!("{} Command not implemented\n", self.code()),
            StatusCode::CmdBadSequence => todo!(),
            StatusCode::CmdNotImplementedParam => todo!(),
            StatusCode::UserNotLoggedIn => todo!(),
            StatusCode::NeedAccountForStore => todo!(),
            StatusCode::ActionNotTaken => todo!(),
            StatusCode::ActionAbortedPageTypeUnknown => todo!(),
            StatusCode::ExceededStorageAllocation => todo!(),
            StatusCode::FilenameNotAllowed => todo!(),
        }
    }
}

#[derive(Debug, Default)]
pub enum DataConnectionType {
    #[default]
    Active,
    Passive,
}

#[derive(Debug)]
pub struct DataConnection {
    conn_type: DataConnectionType,
    address: SocketAddr,
}

pub struct ClientConnection {
    data_connection: Option<DataConnection>,
}

impl ClientConnection {
    pub fn new() -> Self {
        Self {
            data_connection: None,
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
pub enum CommandError {
    #[error("invalid arguments")]
    #[diagnostic(code(ftp::command::invalid_arguments))]
    InvalidArguments,
}

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
    pub fn run(&self) -> Result<StatusCode, CommandError> {
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
