use std::{
    io::{self, stdout},
    sync::mpsc,
    thread,
};

use color_eyre::owo_colors::OwoColorize;
use crossterm::{
    event::{self, *},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use miette::*;
use ratatui::{prelude::*, widgets::*};
use tracing::trace;
use tui_logger::*;

pub struct App {
    mode: AppMode,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    #[default]
    Run,
    Quit,
}

pub enum AppEvent {
    UIEvent(Event),
}

impl App {
    pub fn new() -> Self {
        Self { mode: AppMode::Run }
    }

    pub fn start(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();

        thread::spawn(move || input_thread(event_tx));
        // thread::spawn(move || progress_task(progress_tx).unwrap());
        // thread::spawn(move || background_task());

        self.run(terminal, rx)
    }

    fn run(
        &mut self,
        terminal: &mut Terminal<impl Backend>,
        rx: mpsc::Receiver<AppEvent>,
    ) -> Result<()> {
        for event in rx {
            match event {
                AppEvent::UIEvent(event) => {
                    if let Event::Key(key) = event {
                        if let KeyCode::Char('q') = key.code {
                            self.mode = AppMode::Quit;
                        }
                    }
                }
            }
            if self.mode == AppMode::Quit {
                break;
            }
            self.draw(terminal)?;
        }
        Ok(())
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal
            .draw(|frame| frame.render_widget(self, frame.size()))
            .into_diagnostic()?;
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .title("FTP Server")
            .border_style(Color::Green)
            .render(area, buf);

        let area = area.inner(&Margin {
            horizontal: 1,
            vertical: 1,
        });

        let [body, footer] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(area);

        let [left, _right] =
            Layout::horizontal([Constraint::Fill(75), Constraint::Fill(25)]).areas(body);

        TuiLoggerSmartWidget::default().render(left, buf);

        Paragraph::new("Press 'q' to quit")
            .centered()
            .style(Color::Gray)
            .render(footer, buf);
    }
}

pub fn init_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode().into_diagnostic()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture).into_diagnostic()?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend).into_diagnostic()?;
    Ok(terminal)
}

pub fn restore_terminal() -> Result<()> {
    trace!(target:"crossterm", "Restoring terminal");
    disable_raw_mode().into_diagnostic()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture).into_diagnostic()?;
    Ok(())
}

pub fn input_thread(tx_event: mpsc::Sender<AppEvent>) -> Result<()> {
    trace!(target:"crossterm", "Starting input thread");
    while let Ok(event) = event::read() {
        trace!(target:"crossterm", "Stdin event received {:?}", event);
        tx_event.send(AppEvent::UIEvent(event)).into_diagnostic()?;
    }
    Ok(())
}
