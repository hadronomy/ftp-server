use miette::*;
use tokio::net::tcp::WriteHalf;

use crate::{FTPCommand, InnerConnectionRef, StatusCode};

pub struct Pass<'a>(&'a str);

impl<'a> FTPCommand<'a> for Pass<'a> {
    const KEYWORD: &'static str = "PASS";

    async fn run<'b>(
        &self,
        _connection: InnerConnectionRef,
        _writer: &mut WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        Ok(Some(StatusCode::UserLoggedIn))
    }
}

impl<'a> TryFrom<(&'a str, Vec<&'a str>)> for Pass<'a> {
    type Error = miette::Error;

    fn try_from((command, args): (&'a str, Vec<&'a str>)) -> Result<Self> {
        if command == Self::KEYWORD {
            if (0..=1).contains(&args.len()) {
                let password = args.first().copied().unwrap_or("anonymous");
                Ok(Self(password))
            } else {
                Err(miette!("Invalid number of arguments"))
            }
        } else {
            Err(miette!("Invalid command"))
        }
    }
}
