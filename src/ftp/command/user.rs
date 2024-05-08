use miette::*;
use tokio::net::tcp::WriteHalf;

use crate::{FTPCommand, InnerConnectionRef, StatusCode};

pub struct User<'a>(&'a str);

impl<'a> FTPCommand<'a> for User<'a> {
    const KEYWORD: &'static str = "USER";

    async fn run<'b>(
        &self,
        _connection: InnerConnectionRef,
        _writer: &mut WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        Ok(Some(StatusCode::UsernameOkNeedPassword))
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
