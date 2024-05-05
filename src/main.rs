mod app;
mod cli;
mod ftp;
mod parser;

use std::io;
use std::net::SocketAddr;

use miette::*;
use tracing::*;
use tracing_subscriber::prelude::*;

use crate::app::*;
use crate::cli::*;
use crate::ftp::*;

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    if let Some(cli) = Args::init_cli() {
        let (non_blocking, _guard) = tracing_appender::non_blocking(io::stdout());

        if cli.interactive {
            tracing_subscriber::registry()
                .with(tui_logger::tracing_subscriber_layer())
                .init();
        } else {
            tracing_subscriber::registry()
                .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
                .init();
        }

        if cfg!(debug_assertions) {
            warn!("You are currently running a debug build");
        }

        if cli.interactive {
            info!("Starting FTP server");
            warn!("Currently interactive mode is WIP");

            let mut terminal = init_terminal()?;
            terminal.hide_cursor().into_diagnostic()?;
            terminal.clear().into_diagnostic()?;

            let mut app = App::default();
            app.start(&mut terminal)?;
            terminal.show_cursor().into_diagnostic()?;

            restore_terminal()?;
        } else {
            let addr = SocketAddr::from(([127, 0, 0, 1], cli.port));
            let mut server = FTPServer::from((addr, cli.data_port));
            server.listen().await?;
        }
    }
    Ok(())
}
