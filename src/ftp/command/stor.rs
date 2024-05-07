use miette::*;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use tracing::*;

use crate::{FTPCommand, InnerConnectionRef, StatusCode};

pub struct Stor<'a>(&'a str);

impl<'a> FTPCommand<'a> for Stor<'a> {
    const KEYWORD: &'static str = "STOR";

    async fn run<'b>(
        &self,
        connection: InnerConnectionRef,
        writer: &mut tokio::net::tcp::WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        let destination = self.0;

        writer
            .write(StatusCode::DataOpenTransfer.to_string().as_bytes())
            .await
            .into_diagnostic()?;

        let connection = connection.lock().await;

        let data_connection = connection.data_connection.as_ref().unwrap();
        let mut data_connection = data_connection.lock().await;

        let path = connection.cwd().join(destination);
        let mut file = File::create(path).await.into_diagnostic()?;

        let mut buffer = vec![0; 4096];
        loop {
            let bytes_read = data_connection.read(&mut buffer).await.into_diagnostic()?;
            if bytes_read == 0 {
                break;
            }
            file.write_all(&buffer[..bytes_read])
                .await
                .into_diagnostic()?;
        }
        data_connection.shutdown().await.into_diagnostic()?;

        debug!("Data received");

        Ok(Some(StatusCode::CantOpenDataConnection))
    }
}

impl<'a> TryFrom<(&'a str, Vec<&'a str>)> for Stor<'a> {
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
