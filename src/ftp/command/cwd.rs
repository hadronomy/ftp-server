use std::{env, path::Path, sync::Arc};

use miette::*;

use tokio::{net::tcp::WriteHalf, sync::Mutex};
use tracing::*;

use super::{FTPCommand, InnerConnection, StatusCode};

pub struct Cwd<'a>(&'a str);

impl<'a> FTPCommand<'a> for Cwd<'a> {
    const KEYWORD: &'static str = "CWD";

    async fn run<'b>(
        &self,
        _connection: Arc<Mutex<InnerConnection>>,
        _writer: &mut WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        trace!("Changing working directory");
        let new_cwd = Path::new(self.0);
        env::set_current_dir(new_cwd).into_diagnostic()?;

        Ok(Some(StatusCode::FileActionOk(
            "Directory successfully changed".to_string(),
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
