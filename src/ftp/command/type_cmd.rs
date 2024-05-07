use miette::*;

use tracing::*;

use crate::{FTPCommand, InnerConnectionRef, StatusCode};

pub struct Type(char);

impl<'a> FTPCommand<'a> for Type {
    const KEYWORD: &'static str = "TYPE";

    #[tracing::instrument(skip(self, _connection, _writer))]
    async fn run<'b>(
        &self,
        _connection: InnerConnectionRef,
        _writer: &mut tokio::net::tcp::WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        trace!("Setting transfer type to {}", self.0);
        Ok(Some(StatusCode::Ok))
    }
}

impl<'a> TryFrom<(&'a str, Vec<&'a str>)> for Type {
    type Error = miette::Error;

    fn try_from((command, args): (&'a str, Vec<&'a str>)) -> Result<Self> {
        if command == Self::KEYWORD {
            if args.len() == 1 {
                Ok(Self(args[0].chars().next().unwrap()))
            } else {
                Err(miette!("Invalid number of arguments"))
            }
        } else {
            Err(miette!("Invalid command"))
        }
    }
}
