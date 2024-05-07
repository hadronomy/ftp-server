use miette::*;

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

        let path = connection.lock().await.cwd().join(source);
        trace!("Opening file {:?}", path);
        let mut file = match File::open(path).await.into_diagnostic() {
            Ok(file) => file,
            Err(_) => {
                error!("File not found");
                return Ok(Some(StatusCode::FileActionNotTaken));
            }
        };

        writer
            .write(StatusCode::DataOpenTransfer.to_string().as_bytes())
            .await
            .into_diagnostic()?;

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
        data_connection.shutdown().await.into_diagnostic()?;

        debug!("Data sent");

        Ok(Some(StatusCode::ClosingDataConnection))
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
