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

        writer
            .write(StatusCode::DataOpenTransfer.to_string().as_bytes())
            .await
            .into_diagnostic()?;

        let mut file = File::open(source).await.into_diagnostic()?;
        let mut buf = String::new();
        file.read_to_string(&mut buf).await.into_diagnostic()?;
        buf = buf.replace('\n', "\r\n");

        trace!("Read data: {:?}", buf);
        let connection = connection.lock().await;
        let data_connection = connection.data_connection.as_ref().unwrap();
        let mut data_connection = data_connection.lock().await;
        trace!("Sending data");
        data_connection.send(&buf.into_bytes()).await?;
        trace!("Data sent");

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
