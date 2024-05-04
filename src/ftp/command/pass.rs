use miette::*;
use tokio::net::tcp::WriteHalf;

use super::{Connection, FTPCommand, StatusCode};

pub struct Pass<'a>(&'a str);

impl<'a> FTPCommand<'a> for Pass<'a> {
    const KEYWORD: &'static str = "PASS";

    async fn run<'b>(
        &self,
        _connection: &mut Connection,
        _writer: &mut WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        Ok(Some(StatusCode::UserLoggedIn))
    }
}

impl<'a> TryFrom<(&'a str, Vec<&'a str>)> for Pass<'a> {
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
