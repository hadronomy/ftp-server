use miette::*;
use ratatui::buffer;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::tcp::WriteHalf,
};
use tracing::*;

use crate::{FTPCommand, InnerConnectionRef, StatusCode};

pub struct Retr<'a>(&'a str);

impl<'a> FTPCommand<'a> for Retr<'a> {
    const KEYWORD: &'static str = "RETR";

    async fn run<'b>(
        &self,
        connection: InnerConnectionRef,
        writer: &mut WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        let source = self.0;

        writer
            .write(StatusCode::DataOpenTransfer.to_string().as_bytes())
            .await
            .into_diagnostic()?;

        let mut file = File::open(source).await.into_diagnostic()?;

        let connection = connection.lock().await;
        let data_connection = connection.data_connection.as_ref().unwrap();
        let mut data_connection = data_connection.lock().await;

        let mut buffer = vec![0; 4096];
        loop {
            let bytes_read = file.read(&mut buffer).await.into_diagnostic()?;
            if bytes_read == 0 {
                break;
            }
            data_connection
                .write_all(&buffer[..bytes_read])
                .await
                .into_diagnostic()?;
        }
        data_connection.flush().await.into_diagnostic()?;
        data_connection.shutdown().await.into_diagnostic()?;

        debug!("Data sent");

        Ok(Some(StatusCode::Ok))
    }
}

impl<'a> TryFrom<(&'a str, Vec<&'a str>)> for Retr<'a> {
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
