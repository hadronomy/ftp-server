use std::str;

use miette::*;
use tokio::{fs::File, io::AsyncWriteExt};
use tracing::*;

use super::{Connection, FTPCommand, StatusCode};

pub struct Stor<'a>(&'a str);

impl<'a> FTPCommand<'a> for Stor<'a> {
    const KEYWORD: &'static str = "STOR";

    async fn run<'b>(
        &self,
        connection: &mut Connection,
        writer: &mut tokio::net::tcp::WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        let destination = self.0;

        writer
            .write(StatusCode::DataOpenTransfer.to_string().as_bytes())
            .await
            .into_diagnostic()?;

        let data_connection = connection.data_connection.as_ref().unwrap();
        let mut data_connection = data_connection.lock().await;
        let data = data_connection.receive().await?;
        trace!("Received data: {:?}", str::from_utf8(&data).unwrap());

        let mut file = File::create(destination).await.into_diagnostic()?;
        file.write(&data).await.into_diagnostic()?;

        Ok(Some(StatusCode::Ok))
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
