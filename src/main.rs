mod app;
mod cli;
mod display;
mod events;
mod file;
mod logging;
mod movement;
mod popup;

use app::App;
use clap::Parser;
use log::{debug, error, info};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use std::io;
use std::process;

use logging::init_logs;

fn main() {
    init_logs();
    debug!("logs initialized successfully");

    if let Err(err) = actual_main() {
        error!("error: {err}");
        process::exit(1);
    }

    info!("Bye!");
}

fn actual_main() -> io::Result<()> {
    let args = cli::Args::parse();

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    let term_size = terminal.size()?;
    debug!("terminal size: {term_size}");
    let mut app = App::new(
        args.file,
        args.blocksize,
        (term_size.width, term_size.height),
    )?;
    debug!("app initialized successfully");

    init_terminal_state()?;
    let res = app.run(&mut terminal);
    cleanup_terminal_state()?;

    res
}

fn init_terminal_state() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    debug!("terminal state initialized");
    Ok(())
}

fn cleanup_terminal_state() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    debug!("terminal state cleaned");
    Ok(())
}
