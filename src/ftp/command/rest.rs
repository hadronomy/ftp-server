use std::sync::Arc;

use miette::*;

use tokio::{net::tcp::WriteHalf, sync::Mutex};
use tracing::*;

use super::{FTPCommand, InnerConnection, StatusCode};

pub struct Rest(u64);

impl<'a> FTPCommand<'a> for Rest {
    const KEYWORD: &'static str = "REST";

    #[tracing::instrument(skip(self, _connection, _writer))]
    async fn run<'b>(
        &self,
        _connection: Arc<Mutex<InnerConnection>>,
        _writer: &mut WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        trace!("Restarting at {}", self.0);
        Ok(Some(StatusCode::FileActionPending))
    }
}

impl<'a> TryFrom<(&'a str, Vec<&'a str>)> for Rest {
    type Error = miette::Error;

    fn try_from((command, args): (&'a str, Vec<&'a str>)) -> Result<Self> {
        if command == Self::KEYWORD {
            if args.len() == 1 {
                Ok(Self(args[0].parse().into_diagnostic()?))
            } else {
                Err(miette!("Invalid number of arguments"))
            }
        } else {
            Err(miette!("Invalid command"))
        }
    }
}
