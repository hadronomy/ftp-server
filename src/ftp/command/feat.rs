use miette::*;
use tracing::*;

use crate::{FTPCommand, InnerConnectionRef, StatusCode};

pub struct Feat;

impl<'a> FTPCommand<'a> for Feat {
    const KEYWORD: &'static str = "FEAT";

    async fn run<'b>(
        &self,
        _connection: InnerConnectionRef,
        _writer: &mut tokio::net::tcp::WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        trace!("Reporting supported features");
        Ok(Some(StatusCode::SystemStatus(
            "-Features:
 MLST
 MLSD
 UTF8\
"
            .to_string(),
        )))
    }
}

impl<'a> TryFrom<(&'a str, Vec<&'a str>)> for Feat {
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
