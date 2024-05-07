use std::os::{linux::fs::MetadataExt, unix::fs::PermissionsExt};

use chrono::DateTime;
use miette::*;

use tokio::{io::AsyncWriteExt, net::tcp::WriteHalf};
use tracing::*;

use crate::utils::permissions_to_string;

use crate::{FTPCommand, InnerConnectionRef, StatusCode};

pub struct List<'a>(Vec<&'a str>);

impl<'a> FTPCommand<'a> for List<'a> {
    const KEYWORD: &'static str = "LIST";

    async fn run<'b>(
        &self,
        connection: InnerConnectionRef,
        writer: &mut WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        writer
            .write(StatusCode::DataOpenTransfer.to_string().as_bytes())
            .await
            .into_diagnostic()?;

        while connection.lock().await.data_connection.as_ref().is_none() {
            trace!("Waiting for data connection");
            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
        }

        let connection = connection.lock().await;
        if let Some(data_connection) = connection.data_connection.as_ref() {
            let mut data_connection = data_connection.lock().await;
            for entry in
                std::fs::read_dir(std::env::current_dir().into_diagnostic()?).into_diagnostic()?
            {
                trace!("Reading entry {:?}", entry);
                let entry = entry.into_diagnostic()?;
                let metadata = entry.metadata().into_diagnostic()?;
                let file_type = if metadata.is_dir() { "d" } else { "-" };
                let permissions = permissions_to_string(metadata.permissions().mode());
                let links = metadata.st_nlink();
                let user = metadata.st_uid();
                let group = metadata.st_gid();
                let date = metadata.modified().into_diagnostic()?;
                let formated_date = DateTime::<chrono::Local>::from(date).format("%e %b %y %H:%M");
                let name = entry.file_name();
                let name = name.to_string_lossy();
                let line = format!(
                    "{}{} {} {} {} {} {}\r\n",
                    file_type, permissions, links, user, group, formated_date, name
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
            data_connection.flush().await.into_diagnostic()?;
            data_connection.shutdown().await.into_diagnostic()?;
        }

        Ok(Some(StatusCode::ClosingDataConnection))
    }
}

impl<'a> TryFrom<(&'a str, Vec<&'a str>)> for List<'a> {
    type Error = miette::Error;

    fn try_from((command, args): (&'a str, Vec<&'a str>)) -> Result<Self> {
        if command == Self::KEYWORD {
            Ok(Self(args))
        } else {
            Err(miette!("Invalid command"))
        }
    }
}
