use std::{ffi::OsString};

use miette::*;

use tokio::net::tcp::WriteHalf;
use tracing::*;

use crate::{FTPCommand, InnerConnectionRef, StatusCode};

pub struct Cwd<'a>(&'a str);

impl<'a> FTPCommand<'a> for Cwd<'a> {
    const KEYWORD: &'static str = "CWD";

    async fn run<'b>(
        &self,
        connection: InnerConnectionRef,
        _writer: &mut WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        trace!("Changing working directory");
        let new_cwd = OsString::from(self.0);
        trace!("New CWD: {:?}", new_cwd);
        connection.lock().await.change_dir(new_cwd).await?;

        Ok(Some(StatusCode::FileActionOk(
            " Directory successfully changed".to_string(),
        )))
    }
}

impl<'a> TryFrom<(&'a str, Vec<&'a str>)> for Cwd<'a> {
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
