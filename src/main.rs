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
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use std::io::{self, Write};
use std::process;

fn main() {
    if let Err(err) = actual_main() {
        let _ = writeln!(io::stderr(), "error: {err}");
        process::exit(1);
    }
}

fn actual_main() -> io::Result<()> {
    let args = cli::Args::parse();

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    let term_size = terminal.size()?;
    let mut app = App::new(
        args.file,
        args.blocksize,
        (term_size.width, term_size.height),
    )?;

    init_terminal_state()?;
    let res = app.run(&mut terminal);
    cleanup_terminal_state()?;

    res
}

fn init_terminal_state() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    Ok(())
}

fn cleanup_terminal_state() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
