use miette::*;

use crate::ftp::StatusCode;
use crate::Connection;

pub trait Runnable {
    fn run(&self, connection: &mut Connection) -> Result<StatusCode>;
}

pub trait FTPCommand<'a>
where
    Self: TryFrom<(&'a str, Vec<&'a str>)>,
{
    const KEYWORD: &'static str;

    fn execute(&self, connection: &mut Connection) -> Result<StatusCode>;

    fn is_keyword(&self, command: &str) -> bool {
        command == Self::KEYWORD
    }
}

impl<'a, T> Runnable for T
where
    T: FTPCommand<'a>,
{
    fn run(&self, connection: &mut Connection) -> Result<StatusCode> {
        self.execute(connection)
    }
}

struct User<'a>(&'a str);

impl<'a> FTPCommand<'a> for User<'a> {
    const KEYWORD: &'static str = "USER";

    fn execute(&self, _connection: &mut Connection) -> Result<StatusCode> {
        Ok(StatusCode::UserLoggedIn)
    }
}

impl<'a> TryFrom<(&'a str, Vec<&'a str>)> for User<'a> {
    type Error = miette::Error;

    fn try_from((command, args): (&'a str, Vec<&'a str>)) -> Result<Self> {
        if command == Self::KEYWORD {
            if args.len() == 1 {
                Ok(Self(args[0]))
            } else {
                Err(miette!("Invalid number of arguments"))
            }
        } else {
            Err(miette!("Invalid command"))
        }
    }
}

/// The FTP commands
///
/// See [RFC 959](https://tools.ietf.org/html/rfc959)
enum Command<'a> {
    User(User<'a>),
}

impl<'a> Runnable for Command<'a> {
    fn run(&self, connection: &mut Connection) -> Result<StatusCode> {
        match self {
            Command::User(user) => user.run(connection),
        }
    }
}

impl<'a> TryFrom<(&'a str, Vec<&'a str>)> for Command<'a> {
    type Error = miette::Error;

    fn try_from((command, args): (&'a str, Vec<&'a str>)) -> Result<Self> {
        match command {
            User::KEYWORD => Ok(Command::User(User::try_from((command, args))?)),
            _ => Err(miette!("Invalid command")),
        }
    }
}
