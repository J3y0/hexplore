use simplelog::{ColorChoice, LevelFilter, TerminalMode};
use simplelog::{CombinedLogger, ConfigBuilder, SharedLogger, TermLogger, WriteLogger};
use std::env;
use std::fs::OpenOptions;

const LOG_FILE: &str = "hexplore.log";

pub fn init_logs() {
    let mut path = env::temp_dir();
    path.push(LOG_FILE);

    let config_termlogger = ConfigBuilder::new()
        .set_time_level(log::LevelFilter::Off)
        .set_thread_level(log::LevelFilter::Off)
        .set_target_level(log::LevelFilter::Off)
        .set_max_level(log::LevelFilter::Debug)
        .build();
    let config_writelogger = ConfigBuilder::new()
        .set_time_format_rfc2822()
        .set_thread_level(log::LevelFilter::Off)
        .build();

    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
        LevelFilter::Info,
        config_termlogger,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )];

    if let Ok(logfile) = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .append(false)
        .open(path)
    {
        loggers.push(WriteLogger::new(
            LevelFilter::Debug,
            config_writelogger,
            logfile,
        ));
    }

    let _ = CombinedLogger::init(loggers);
}
