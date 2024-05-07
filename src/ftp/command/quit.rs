use miette::*;

use tokio::{io::AsyncWriteExt, net::tcp::WriteHalf};
use tracing::*;

use crate::{DataConnection, FTPCommand, InnerConnectionRef, StatusCode};

pub struct Quit;

impl<'a> FTPCommand<'a> for Quit {
    const KEYWORD: &'static str = "QUIT";

    async fn run<'b>(
        &self,
        _connection: InnerConnectionRef,
        writer: &mut WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        writer
            .write(
                StatusCode::ServiceClosingControlConnection
                    .to_string()
                    .as_bytes(),
            )
            .await
            .into_diagnostic()?;
        writer.shutdown().await.into_diagnostic()?;

        Ok(None)
    }
}

impl<'a> TryFrom<(&'a str, Vec<&'a str>)> for Quit {
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
