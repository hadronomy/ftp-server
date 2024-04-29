use miette::*;
use num_integer::Integer;
use std::{
    borrow::{Borrow, BorrowMut},
    io,
    net::SocketAddr,
    str,
    sync::Arc,
};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufStream},
    net::{tcp::WriteHalf, TcpListener, TcpStream},
    sync::{broadcast, Mutex},
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
            self.add_connection(connection).await?;
        }
    }

    async fn add_connection(&mut self, mut connection: Connection) -> Result<()> {
        // let mut connections = self
        //     .client_connections
        //     .lock()
        //     .expect("Could not lock connections");
        info!(
            "New connection from {}",
            connection.socket.lock().await.peer_addr().unwrap()
        );
        // connections.push(connection);
        tokio::spawn(async move {
            trace!("Spawning new control connection task");
            connection.connect().await.unwrap();
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

#[derive(Debug, Clone)]
pub struct Connection {
    socket: Arc<Mutex<TcpStream>>,
    passive_addr: SocketAddr,
    data_connection: Option<Arc<Mutex<DataConnection>>>,
    // receiver: Arc<Mutex<broadcast::Receiver<String>>>,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket: Arc::new(Mutex::new(socket)),
            passive_addr: SocketAddr::from(([127, 0, 0, 1], 2222)),
            data_connection: None,
        }
    }

    async fn connect(&mut self) -> Result<()> {
        let _addr = self.socket.lock().await.peer_addr().unwrap();
        let socket_clone = self.socket.clone();
        let mut socket_mutex = socket_clone.lock().await;
        let socket = socket_mutex.borrow_mut();
        let (mut read_stream, mut write_stream) = socket.split();
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

            let response = self.execute_command(cmd, args, &mut write_stream).await;
            match response {
                Ok(res) => {
                    if cmd == "QUIT" {
                        debug!("Quitting connection {}", socket.peer_addr().unwrap());
                        return Ok(());
                    }
                    if let Some(res) = res {
                        write_stream
                            .write(res.to_string().as_bytes())
                            .await
                            .into_diagnostic()?;
                    }
                }
                Err(e) => {
                    error!("Error executing command: {:?}", e);
                }
            }

            debug!("Clearing buffer");
            buf.clear();
        }
    }

    async fn execute_command<'a>(
        &mut self,
        cmd: &str,
        args: Vec<&str>,
        writer: &mut WriteHalf<'a>,
    ) -> Result<Option<StatusCode>> {
        match cmd {
            "USER" => Ok(Some(StatusCode::UserLoggedIn)),
            "PASS" => Ok(Some(StatusCode::UserLoggedIn)),
            "SYST" => Ok(Some(StatusCode::SystemType(SystemType::from_os()))),
            "FEAT" => Ok(Some(StatusCode::CmdNotImplemented)),
            "SIZE" => Ok(Some(StatusCode::CmdNotImplemented)),
            "PASV" => {
                let data_addr = self.passive_addr;
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
                self.data_connection = Some(data_connection);

                trace!("Data connection established");

                Ok(None)
            }
            "TYPE" => Ok(Some(StatusCode::CmdNotImplemented)),
            "LPRT" => Ok(Some(StatusCode::CmdNotImplemented)),
            "PORT" => {
                // read port from args
                let address = args.first().unwrap();
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
                self.data_connection = Some(data_connection);

                Ok(Some(StatusCode::Ok))
            }
            "STOR" => {
                let destination = args.first().unwrap();

                writer
                    .write(StatusCode::DataOpenTransfer.to_string().as_bytes())
                    .await
                    .into_diagnostic()?;

                let data_connection = self.data_connection.as_ref().unwrap();
                let mut data_connection = data_connection.lock().await;
                let data = data_connection.receive().await?;
                trace!("Received data: {:?}", str::from_utf8(&data).unwrap());

                let mut file = File::create(destination).await.into_diagnostic()?;
                file.write(&data).await.into_diagnostic()?;

                Ok(Some(StatusCode::Ok))
            }
            "RETR" => {
                let source = args.first().unwrap();

                writer
                    .write(StatusCode::DataOpenTransfer.to_string().as_bytes())
                    .await
                    .into_diagnostic()?;

                let mut file = File::open(source).await.into_diagnostic()?;
                let mut buf = String::new();
                file.read_to_string(&mut buf).await.into_diagnostic()?;
                buf = buf.replace('\n', "\r\n");

                trace!("Read data: {:?}", buf);

                let data_connection = self.data_connection.as_ref().unwrap();
                let mut data_connection = data_connection.lock().await;
                trace!("Sending data");
                data_connection.send(&buf.into_bytes()).await?;
                trace!("Data sent");

                Ok(Some(StatusCode::PathCreated))
            }
            "QUIT" => {
                writer
                    .write(StatusCode::ServiceClosingControlConn.to_string().as_bytes())
                    .await
                    .into_diagnostic()?;

                writer.flush().await.into_diagnostic()?;
                writer.shutdown().await.into_diagnostic()?;

                Ok(Some(StatusCode::Ok))
            }
            _ => Ok(Some(StatusCode::CmdNotImplemented)),
        }
    }
}

#[derive(Debug)]
pub struct DataConnection {
    socket: TcpStream,
}

impl DataConnection {
    pub async fn send(&mut self, data: &[u8]) -> Result<()> {
        self.socket.write_all(data).await.into_diagnostic()?;
        self.socket.shutdown().await.into_diagnostic()?;
        Ok(())
    }

    pub async fn receive(&mut self) -> Result<Vec<u8>> {
        let mut reader = BufReader::new(&mut self.socket);
        let mut buf = Vec::new();
        reader.read_until(b'\0', &mut buf).await.into_diagnostic()?;
        Ok(buf)
    }
}

impl From<TcpStream> for DataConnection {
    fn from(socket: TcpStream) -> Self {
        Self { socket }
    }
}
