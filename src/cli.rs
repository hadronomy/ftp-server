use clap::{CommandFactory, Parser};
use clap_help::Printer;
use termimad::ansi;

static INTRO: &str = "

An interactive FTPServer

";

#[derive(Parser, Debug)]
#[command(name = "ftpy", author, version, about, disable_help_flag = true)]
pub struct Args {
    /// Print help
    #[arg(long)]
    pub help: bool,

    /// Interactive mode, that is, the cool mode
    #[arg(short, long)]
    pub interactive: bool,
}

/// Implements the `Args` struct and its associated methods.
impl Args {
    /// Initializes the command-line interface (CLI) and returns an `Option<Self>` object.
    ///
    /// # Returns
    ///
    /// - `Some(Self)`: If the CLI arguments were successfully parsed.
    /// - `None`: If the `help` flag is set, the help message is printed and `None` is returned.
    pub fn init_cli() -> Option<Self> {
        let args = Self::parse();
        if args.help {
            Self::print_help();
            return None;
        }
        Some(args)
    }

    pub fn print_help() {
        let mut printer = Printer::new(Args::command())
            .with("introduction", INTRO)
            .with("options", clap_help::TEMPLATE_OPTIONS_MERGED_VALUE)
            .without("author");
        let skin = printer.skin_mut();
        let color = ansi(80);
        skin.headers[0].compound_style.set_fg(color);
        skin.bold.set_fg(color);
        skin.italic = termimad::CompoundStyle::with_fg(color);
        printer.print_help();
    }
}
