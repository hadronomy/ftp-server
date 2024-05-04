use miette::*;
use tokio::net::tcp::WriteHalf;

use crate::ftp::StatusCode;
use crate::Connection;

use self::pass::Pass;
use self::pasv::Pasv;
use self::port::Port;
use self::retr::Retr;
use self::stor::Stor;
use self::syst::Syst;
use self::user::User;

mod pass;
mod pasv;
mod port;
mod retr;
mod stor;
mod syst;
mod user;

pub trait FTPCommand<'a>
where
    Self: TryFrom<(&'a str, Vec<&'a str>)>,
{
    const KEYWORD: &'static str;

    async fn run<'b>(
        &self,
        connection: &mut Connection,
        writer: &mut WriteHalf<'b>,
    ) -> Result<Option<StatusCode>>;

    fn is_keyword(&self, command: &str) -> bool {
        command == Self::KEYWORD
    }
}

/// The FTP commands
///
/// See [RFC 959](https://tools.ietf.org/html/rfc959)
pub enum Command<'a> {
    User(User<'a>),
    Pass(Pass<'a>),
    Pasv(Pasv),
    Stor(Stor<'a>),
    Retr(Retr<'a>),
    Port(Port<'a>),
    Syst(Syst),
}

impl<'a> Command<'a> {
    pub async fn run<'b>(
        &self,
        connection: &mut Connection,
        writer: &mut WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        match self {
            Command::User(cmd) => cmd.run(connection, writer).await,
            Command::Pass(cmd) => cmd.run(connection, writer).await,
            Command::Pasv(cmd) => cmd.run(connection, writer).await,
            Command::Stor(cmd) => cmd.run(connection, writer).await,
            Command::Retr(cmd) => cmd.run(connection, writer).await,
            Command::Port(cmd) => cmd.run(connection, writer).await,
            Command::Syst(cmd) => cmd.run(connection, writer).await,
        }
    }
}

impl<'a> TryFrom<(&'a str, Vec<&'a str>)> for Command<'a> {
    type Error = miette::Error;

    fn try_from((command, args): (&'a str, Vec<&'a str>)) -> Result<Self> {
        match command {
            User::KEYWORD => Ok(Command::User(User::try_from((command, args))?)),
            Pass::KEYWORD => Ok(Command::Pass(Pass::try_from((command, args))?)),
            Pasv::KEYWORD => Ok(Command::Pasv(Pasv::try_from((command, args))?)),
            Stor::KEYWORD => Ok(Command::Stor(Stor::try_from((command, args))?)),
            Retr::KEYWORD => Ok(Command::Retr(Retr::try_from((command, args))?)),
            Port::KEYWORD => Ok(Command::Port(Port::try_from((command, args))?)),
            Syst::KEYWORD => Ok(Command::Syst(Syst::try_from((command, args))?)),
            _ => bail!("Invalid command"),
        }
    }
}
