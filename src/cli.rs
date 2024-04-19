use clap::{CommandFactory, Parser};
use clap_help::Printer;
use termimad::ansi;

static INTRO: &str = "

*ftpy* is an interactive FTPServer

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

    #[cfg_attr(debug_assertions, arg(short, long, default_value = "2121"))]
    #[cfg_attr(not(debug_assertions), arg(short, long, default_value = "21"))]
    pub port: u16,
}

/// Implements the `Args` struct and its associated methods.
impl Args {
    /// Initializes the command-line interface (CLI) and returns an `Option<Args>` object.
    /// ```
    /// let args = Args::init_cli();
    /// ```
    ///
    /// # Returns
    ///
    /// - `Some(Args)`: If the CLI arguments were successfully parsed.
    /// - `None`: If the `help` flag is set, the help message is printed and `None` is returned.
    pub fn init_cli() -> Option<Self> {
        let args = Self::parse();
        if args.help {
            Self::print_help();
            return None;
        }
        Some(args)
    }

    /// Prints the help message for the CLI.
    ///
    /// The help message is styled using the `clap-help` and
    /// the `termimad` crates.
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
