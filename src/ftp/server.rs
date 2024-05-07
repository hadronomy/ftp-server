//! This module contains the implementation of an FTP server.
//!
//! The `FTPServer` struct represents an FTP server that listens for incoming connections
//! and handles client requests. It has a `listen` method that starts the server and
//! accepts new connections. Each new connection is handled by the `Connection` struct.
//!
//! The `Connection` struct represents a client connection to the FTP server. It handles
//! the communication with the client, executing commands and sending responses. It also
//! manages the data connection for file transfers.
//!
//! The `DataConnection` struct represents a data connection for sending and receiving data.
//! It is used by the `Connection` struct for file transfers.
//!
//! The code also includes various helper functions and enums for handling FTP commands,
//! status codes, and system types.

use std::{borrow::BorrowMut, net::SocketAddr, str, sync::Arc};

use miette::*;

use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
    net::{tcp::WriteHalf, TcpListener, TcpStream},
    sync::Mutex,
};
use tracing::*;

use crate::StatusCode;
use crate::{parser::cmd_parser, Command};

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
            let connection = Connection::from(socket);
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
            connection
                .inner()
                .lock()
                .await
                .socket
                .lock()
                .await
                .peer_addr()
                .unwrap()
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
        Self { addr }
    }
}

#[derive(Debug, Clone)]
// client_connections: Arc::new(Mutex::new(Vec::new())),
pub struct InnerConnection {
    pub(crate) socket: Arc<Mutex<TcpStream>>,
    pub(crate) data_connection: Option<Arc<Mutex<DataConnection>>>,
}

impl InnerConnection {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket: Arc::new(Mutex::new(socket)),
            data_connection: None,
        }
    }
}

pub type InnerConnectionRef = Arc<Mutex<InnerConnection>>;

#[derive(Debug, Clone)]
pub struct Connection {
    inner: InnerConnectionRef,
}

impl Connection {
    pub fn new(inner: InnerConnection) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub fn inner(&self) -> InnerConnectionRef {
        self.inner.clone()
    }

    #[tracing::instrument(skip(self))]
    pub async fn connect(&mut self) -> Result<()> {
        let _addr = self
            .inner
            .lock()
            .await
            .socket
            .lock()
            .await
            .peer_addr()
            .unwrap();
        let socket_clone = self.inner.lock().await.socket.clone();
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
        if let Ok(code) = Command::try_from((cmd, args)) {
            code.run(self.inner.clone(), writer).await
        } else {
            Ok(Some(StatusCode::CmdNotImplemented))
        }
    }
}

impl From<TcpStream> for Connection {
    fn from(socket: TcpStream) -> Self {
        let inner = InnerConnection::new(socket);
        Self::new(inner)
    }
}

#[derive(Debug)]
pub struct DataConnection {
    socket: TcpStream,
}

impl AsyncWrite for DataConnection {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        std::pin::Pin::new(&mut self.get_mut().socket).poll_write(cx, buf)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.get_mut().socket).poll_flush(cx)
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.get_mut().socket).poll_shutdown(cx)
    }
}

impl AsyncRead for DataConnection {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.get_mut().socket).poll_read(cx, buf)
    }
}

impl From<TcpStream> for DataConnection {
    fn from(socket: TcpStream) -> Self {
        Self { socket }
    }
}
