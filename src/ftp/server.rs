use std::{
    io,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use tokio::{
    io::BufStream,
    net::{TcpListener, TcpStream},
};
use tracing::*;

use crate::StatusCode;

#[derive(Debug, Clone)]
struct FTPServer {
    addr: SocketAddr,
    listener: Option<Arc<TcpListener>>,
    data_connections: Arc<Mutex<Vec<DataConnection>>>,
}

impl FTPServer {
    async fn listen(&mut self) -> io::Result<()> {
        todo!("Implement listening for incoming connections");
    }

    async fn handle_command(&'static mut self, stream: TcpStream) -> io::Result<()> {
        todo!("Implement handling of FTP commands");
    }
}

impl From<SocketAddr> for FTPServer {
    fn from(addr: SocketAddr) -> Self {
        Self {
            addr,
            listener: None,
            data_connections: Arc::new(Mutex::new(vec![])),
        }
    }
}

struct Connection {
    socket: (TcpStream, SocketAddr),
    data_connection: Option<Arc<Mutex<DataConnection>>>,
}

impl Connection {

    async fn handle_command(&mut self) -> io::Result<()> {
        todo!("Implement handling of FTP commands");
    }

    async fn handle_data(&mut self) -> io::Result<()> {
        todo!("Implement handling of data connection");
    }
}

impl From<(TcpStream, SocketAddr)> for Connection {
    fn from(socket: (TcpStream, SocketAddr)) -> Self {
        Self {
            socket,
            data_connection: None,
        }
    }
}

#[derive(Debug, Clone)]
struct DataConnection {
    addr: SocketAddr,
    listener: Option<Arc<TcpListener>>,
}

impl DataConnection {
    async fn accept(&self) -> io::Result<TcpStream> {
        todo!("Implement handling of data connection");
    }
}

impl From<SocketAddr> for DataConnection {
    fn from(addr: SocketAddr) -> Self {
        Self {
            addr,
            listener: None,
        }
    }
}
