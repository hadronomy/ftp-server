use chrono::DateTime;
use miette::*;

use tokio::{io::AsyncWriteExt, net::tcp::WriteHalf};
use tracing::*;

use crate::utils::permissions_to_machine_string;

use crate::{FTPCommand, InnerConnectionRef, StatusCode};

pub struct Mlsd<'a>(Vec<&'a str>);

impl<'a> FTPCommand<'a> for Mlsd<'a> {
    const KEYWORD: &'static str = "MLSD";

    async fn run<'b>(
        &self,
        connection: InnerConnectionRef,
        writer: &mut WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        writer
            .write(
                StatusCode::FileStatusOk(" Directory listing has started".to_string())
                    .to_string()
                    .as_bytes(),
            )
            .await
            .into_diagnostic()?;

        let connection = connection.lock().await;
        let path = connection.cwd();
        if let Some(data_connection) = connection.data_connection.as_ref() {
            let mut data_connection = data_connection.lock().await;
            for entry in std::fs::read_dir(path).into_diagnostic()? {
                let entry = entry.into_diagnostic()?;
                let metadata = entry.metadata().into_diagnostic()?;
                let file_type = if metadata.is_dir() { "dir" } else { "file" };
                let date = metadata.modified().into_diagnostic()?;
                let formated_date = DateTime::<chrono::Local>::from(date).format("%Y%m%d%H%M%S");
                let permissions = permissions_to_machine_string(&entry)?;
                let name = entry.file_name();
                let name = name.to_string_lossy();
                let line = format!(
                    "Type={};Size={};Modify={};Perm={} {}\r\n",
                    file_type,
                    metadata.len(),
                    formated_date,
                    permissions,
                    name
                );
                trace!("Sending line: {}", line.trim());
                data_connection
                    .write(line.as_bytes())
                    .await
                    .into_diagnostic()?;
            }
            data_connection
                .write("\0".as_bytes())
                .await
                .into_diagnostic()?;

            data_connection.shutdown().await.into_diagnostic()?;
        } else {
            return Ok(Some(StatusCode::CantOpenDataConnection));
        }

        trace!("Closing data connection");
        Ok(Some(StatusCode::ClosingDataConnection))
    }
}

impl<'a> TryFrom<(&'a str, Vec<&'a str>)> for Mlsd<'a> {
    type Error = miette::Error;

    fn try_from((_command, args): (&'a str, Vec<&'a str>)) -> Result<Self> {
        Ok(Self(args))
    }
}
