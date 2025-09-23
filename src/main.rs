mod app;
mod cli;
mod display;
mod events;
mod file;
mod movement;
mod popup;

use app::App;
use clap::Parser;
use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::crossterm::event::{self, DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use std::io::{self, Write};
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();

    // ratatui init
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let mut app = App::new(args.file, args.align)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    run_app(&mut terminal, &mut app).unwrap_or_else(|err| {
        let _ = writeln!(io::stderr(), "error occurred: {err:?}");
    });

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> anyhow::Result<()> {
    terminal.draw(|f| app.draw(f))?;
    loop {
        if app.quit {
            break;
        }

        if event::poll(Duration::from_millis(2000))? {
            let event = event::read()?;
            app.handle_event(event)?;
        }
        terminal.draw(|f| app.draw(f))?;
    }

    Ok(())
}
