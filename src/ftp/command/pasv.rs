use std::sync::Arc;

use miette::*;
use num_integer::Integer;
use tokio::{
    io::AsyncWriteExt,
    net::{tcp::WriteHalf, TcpListener},
    sync::Mutex,
};
use tracing::*;

use crate::{Connection, DataConnection, FTPCommand, StatusCode};

pub struct Pasv;

impl<'a> FTPCommand<'a> for Pasv {
    const KEYWORD: &'static str = "PASV";

    async fn run<'b>(
        &self,
        connection: &mut Connection,
        writer: &mut WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        let data_addr = connection.passive_addr;
        let data_port = data_addr.port();
        let (port_high, port_low) = data_port.div_rem(&256);
        let data_listener = TcpListener::bind(data_addr)
            .await
            .unwrap_or_else(|_| panic!("Could not bind to address {}", data_addr));
        // let cwd = cwd.clone();
        trace!("Data connection listener bound to {}", data_addr);

        writer
            .write(
                StatusCode::EnteringPassiveMode {
                    port_high,
                    port_low,
                }
                .to_string()
                .as_bytes(),
            )
            .await
            .into_diagnostic()?;

        writer.flush().await.into_diagnostic()?;

        trace!("Waiting for data connection");

        let (data_socket, _) = data_listener
            .accept()
            .await
            .expect("Error accepting connection to data_socket");

        trace!(
            "Data connection accepted from {}",
            data_socket.peer_addr().unwrap()
        );
        let data_connection = Arc::new(Mutex::new(DataConnection::from(data_socket)));
        connection.data_connection = Some(data_connection);

        trace!("Data connection established");

        Ok(None)
    }
}

impl<'a> TryFrom<(&'a str, Vec<&'a str>)> for Pasv {
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
