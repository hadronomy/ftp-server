use std::net::Ipv4Addr;

use crate::types::SystemType;

/// Status codes for FTP
///
/// # Example
/// ```
/// use ftp::StatusCode;
///
/// let status_code = StatusCode::Ok;
/// assert_eq!(status_code.code(), 200);
/// ```
///
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
    RestartMarker(u64),

    /// **120** - Service ready in **nnn** minutes.
    ServiceReadyIn,

    /// **125** - Data connection already open; transfer starting.
    DataOpenTransfer,

    /// **150** - File status okay; about to open data connection.
    FileStatusOk(String),

    /// **200** - Ok
    Ok,

    /// **202** - Command not implemented, superfluous at this site.
    SuperfluousCmdNotImplemented,

    /// **211** - System status, or system help reply.
    SystemStatus(String),

    /// **212** - Directory status.
    DirectoryStatus,

    /// **213** - File status.
    FileStatus,

    /// **214** - Help message.
    HelpMsg { message: String },

    /// **215** - NAME system type.
    /// Where NAME is an official system name from the list in the Assigned Numbers document.
    SystemType(SystemType),

    /// **220** - Service ready for new user.
    ServiceReadyUser,

    /// **221** - Service closing control connection.
    ServiceClosingControlConnection,

    /// **225** - Data connection open; no transfer in progress.
    DataOpenNoTransfer,

    /// **226** - Closing data connection.
    ClosingDataConnection,

    /// **227** - Entering Passive Mode (h1,h2,h3,h4,p1,p2).
    EnteringPassiveMode {
        ip_address: Ipv4Addr,
        port_high: u16,
        port_low: u16,
    },

    /// **230** - User logged in, proceed.
    UserLoggedIn,

    /// **250** - Requested file action okay, completed.
    FileActionOk(String),

    /// **257** - "PATHNAME" created.
    PathCreated(String),

    /// **331** - User name okay, need password.
    UsernameOk,

    /// **332** - Need account for login.
    NeedLoginAccount,

    /// **350** - Requested file action pending further information.
    FileActionPending,

    /// **421** - Service not available, closing control connection.
    Unnavaidable,

    /// **425** - Can't open data connection.
    CantOpenDataConnection,

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
    /// Returns the code of this [`StatusCode`].
    pub fn code(&self) -> u16 {
        match self {
            StatusCode::RestartMarker(_) => 110,
            StatusCode::ServiceReadyIn => 120,
            StatusCode::DataOpenTransfer => 125,
            StatusCode::FileStatusOk(_) => 150,
            StatusCode::Ok => 200,
            StatusCode::SuperfluousCmdNotImplemented => 202,
            StatusCode::SystemStatus(_) => 211,
            StatusCode::DirectoryStatus => 212,
            StatusCode::FileStatus => 213,
            StatusCode::HelpMsg { message: _ } => 214,
            StatusCode::SystemType(_) => 215,
            StatusCode::ServiceReadyUser => 220,
            StatusCode::ServiceClosingControlConnection => 221,
            StatusCode::DataOpenNoTransfer => 225,
            StatusCode::ClosingDataConnection => 226,
            StatusCode::EnteringPassiveMode {
                ip_address: _,
                port_high: _,
                port_low: _,
            } => 227,
            StatusCode::UserLoggedIn => 230,
            StatusCode::FileActionOk(_) => 250,
            StatusCode::PathCreated(_) => 257,
            StatusCode::UsernameOk => 331,
            StatusCode::NeedLoginAccount => 332,
            StatusCode::FileActionPending => 350,
            StatusCode::Unnavaidable => 421,
            StatusCode::CantOpenDataConnection => 425,
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
            StatusCode::RestartMarker(_) => format!("{} Restart marker reply\n", self.code()),
            StatusCode::ServiceReadyIn => todo!(),
            StatusCode::DataOpenTransfer => format!(
                "{} Data connection already open; transfer starting\n",
                self.code()
            ),
            StatusCode::FileStatusOk(msg) => format!("{}{msg}\n", self.code()),
            StatusCode::Ok => format!("{} Ok\n", self.code()),
            StatusCode::SuperfluousCmdNotImplemented => todo!(),
            StatusCode::SystemStatus(status) => {
                format!("{code}{status} \n{code} END\n", code = self.code())
            }
            StatusCode::DirectoryStatus => todo!(),
            StatusCode::FileStatus => todo!(),
            StatusCode::HelpMsg { message } => format!("{} {}\n", self.code(), message),
            StatusCode::SystemType(system_type) => {
                format!("{} {}\n", self.code(), system_type.to_string())
            }
            StatusCode::ServiceReadyUser => format!("{} Service ready for new user\n", self.code()),
            StatusCode::ServiceClosingControlConnection => {
                format!("{} Service closing control connection\n", self.code())
            }
            StatusCode::DataOpenNoTransfer => format!("{} Data connection open\n", self.code()),
            StatusCode::ClosingDataConnection => {
                format!("{} Closing data connection\n", self.code())
            }
            StatusCode::EnteringPassiveMode {
                ip_address,
                port_high,
                port_low,
            } => {
                let octets = ip_address.octets();
                format!(
                    "{} Entering Passive Mode ({}, {}, {}, {}, {}, {})\n",
                    self.code(),
                    octets[0],
                    octets[1],
                    octets[2],
                    octets[3],
                    port_high,
                    port_low
                )
            }
            StatusCode::UserLoggedIn => "230 User logged in, proceed\n".to_string(),
            StatusCode::FileActionOk(msg) => {
                format!("{}{msg}\n", self.code())
            }
            StatusCode::PathCreated(pathname) => {
                format!("{} \"{pathname}\" created\n", self.code())
            }
            StatusCode::UsernameOk => todo!(),
            StatusCode::NeedLoginAccount => format!("{} Need account for login\n", self.code()),
            StatusCode::FileActionPending => format!(
                "{} Requested file action pending further information\n",
                self.code()
            ),
            StatusCode::Unnavaidable => todo!(),
            StatusCode::CantOpenDataConnection => {
                format!("{} Can't open data connection\n", self.code())
            }
            StatusCode::TransferAborted => todo!(),
            StatusCode::FileActionNotTaken => {
                format!("{} Requested file action not taken\n", self.code())
            }
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
