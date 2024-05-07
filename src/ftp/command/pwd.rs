use miette::*;
use tokio::net::tcp::WriteHalf;

use crate::{FTPCommand, InnerConnectionRef, StatusCode};

pub struct Pwd;

impl<'a> FTPCommand<'a> for Pwd {
    const KEYWORD: &'static str = "PWD";

    async fn run<'b>(
        &self,
        connection: InnerConnectionRef,
        _writer: &mut WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        let cwd = connection.lock().await.cwd();

        Ok(Some(StatusCode::PathCreated(format!(
            "{}",
            cwd.to_string_lossy()
        ))))
    }
}

impl<'a> TryFrom<(&'a str, Vec<&'a str>)> for Pwd {
    type Error = miette::Error;

    fn try_from((command, args): (&'a str, Vec<&'a str>)) -> Result<Self> {
        if command == Self::KEYWORD {
            if args.is_empty() {
                Ok(Self)
            } else {
                Err(miette!("Invalid number of arguments"))
            }
        } else {
            Err(miette!("Invalid command"))
        }
    }
}
