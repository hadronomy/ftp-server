use miette::*;
use num_integer::Integer;
use std::{
    io,
    net::SocketAddr,
    str,
    sync::{Arc, Mutex},
};
use tokio::{
    io::BufStream,
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};
use tracing::*;

use crate::parser::cmd_parser;
use crate::{StatusCode, SystemType};

#[derive(Debug, Clone)]
pub struct FTPServer {
    addr: SocketAddr,
    // client_connections: Arc<Mutex<Vec<Connection>>>,
}

impl FTPServer {
    pub async fn listen(&mut self) -> Result<()> {
        let listener = TcpListener::bind(self.addr).await.into_diagnostic()?;
        info!("Listening on {}", self.addr);
        loop {
            let (socket, _) = listener.accept().await.into_diagnostic()?;
            trace!(
                "Acepted new connection from {}",
                socket.peer_addr().unwrap()
            );
            let connection = Connection::new(socket);
            self.add_connection(connection)?;
        }
    }

    fn add_connection(&mut self, mut connection: Connection) -> Result<()> {
        // let mut connections = self
        //     .client_connections
        //     .lock()
        //     .expect("Could not lock connections");
        info!(
            "New connection from {}",
            connection.socket.peer_addr().unwrap()
        );
        // connections.push(connection);
        tokio::spawn(async move {
            trace!("Spawning new control connection task");
            connection.handle_command().await.unwrap();
        });
        Ok(())
    }
}

impl From<SocketAddr> for FTPServer {
    fn from(addr: SocketAddr) -> Self {
        Self {
            addr,
            // client_connections: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[derive(Debug)]
pub struct Connection {
    socket: TcpStream,
    passive_addr: SocketAddr,
    data_connection: Option<Arc<Mutex<DataConnection>>>,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket,
            passive_addr: SocketAddr::from(([127, 0, 0, 1], 2222)),
            data_connection: None,
        }
    }

    async fn handle_command(&mut self) -> Result<()> {
        let _addr = self.socket.peer_addr().into_diagnostic()?;
        let (mut read_stream, mut write_stream) = self.socket.split();
        let mut reader = BufReader::new(&mut read_stream);

        write_stream
            .write(StatusCode::ServiceReadyUser.to_string().as_bytes())
            .await
            .into_diagnostic()?;

        let mut buf = vec![];
        loop {
            let _ = reader.read_until(b'\n', &mut buf).await.into_diagnostic()?;
            let input = str::from_utf8(&buf).into_diagnostic()?.trim_end();
            debug!("Reading {:?} from stream", input);

            let (_, (cmd, args)) = cmd_parser(input).unwrap();
            info!("Received {:?} command with args: {:?}", cmd, args);

            let response = match cmd {
                "USER" => StatusCode::UserLoggedIn,
                "PASS" => StatusCode::UserLoggedIn,
                "SYST" => StatusCode::SystemType(SystemType::from_os()),
                "FEAT" => StatusCode::CmdNotImplemented,
                "SIZE" => StatusCode::CmdNotImplemented,
                "PASV" => {
                    let data_addr = self.passive_addr;
                    let data_port = data_addr.port();
                    let (port_high, port_low) = data_port.div_rem(&256);
                    let data_listener = TcpListener::bind(data_addr)
                        .await
                        .unwrap_or_else(|_| panic!("Could not bind to address {}", data_addr));
                    write_stream
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
                    // let cwd = cwd.clone();
                    let (data_socket, _) = data_listener
                        .accept()
                        .await
                        .expect("Error accepting connection to data_socket");

                    let data_connection = Arc::new(Mutex::new(DataConnection::from(data_socket)));
                    self.data_connection = Some(data_connection);

                    StatusCode::EnteringPassiveMode {
                        port_high,
                        port_low,
                    }
                }
                "STOR" => StatusCode::DataOpenTransfer,
                "LPRT" => StatusCode::CmdNotImplemented,
                "RETR" => StatusCode::DataOpenTransfer,
                _ => StatusCode::CmdNotImplemented,
            };
            debug!("Clearing buffer");
            buf.clear();

            write_stream
                .write(response.to_string().as_bytes())
                .await
                .into_diagnostic()?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct DataConnection {
    socket: TcpStream,
}

impl DataConnection {
    pub async fn send(&mut self, data: &[u8]) -> Result<()> {
        self.socket.write_all(data).await.into_diagnostic()?;
        Ok(())
    }

    pub async fn receive(&mut self) -> Result<Vec<u8>> {
        let mut reader = BufReader::new(&mut self.socket);
        let mut buf = Vec::new();
        reader.read_until(b'\n', &mut buf).await.into_diagnostic()?;
        Ok(buf)
    }
}

impl From<TcpStream> for DataConnection {
    fn from(socket: TcpStream) -> Self {
        Self { socket }
    }
}
