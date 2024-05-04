use std::{net::SocketAddr, sync::Arc};

use miette::*;

use tokio::{net::TcpStream, sync::Mutex};

use crate::DataConnection;

use super::*;

pub struct Port<'a>(&'a str);

impl<'a> FTPCommand<'a> for Port<'a> {
    const KEYWORD: &'static str = "PORT";

    async fn run<'b>(
        &self,
        connection: &mut Connection,
        _writer: &mut WriteHalf<'b>,
    ) -> Result<Option<StatusCode>> {
        let address = self.0;

        let address = address
            .split(',')
            .map(|e| e.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        let port = (address[4] as u16) << 8 | address[5] as u16;
        let ip = [address[0], address[1], address[2], address[3]];
        let data_addr = SocketAddr::from((ip, port));

        let data_socket = TcpStream::connect(data_addr)
            .await
            .expect("Could not connect to data socket");

        let data_connection = Arc::new(Mutex::new(DataConnection::from(data_socket)));
        connection.data_connection = Some(data_connection);

        Ok(Some(StatusCode::Ok))
    }
}

impl<'a> TryFrom<(&'a str, Vec<&'a str>)> for Port<'a> {
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
