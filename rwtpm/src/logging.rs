use std::{
    fs::{File, OpenOptions},
    process::exit,
};
use tracing::{error, level_filters::LevelFilter};
use tracing_subscriber::{fmt, EnvFilter};

/// This function initializes logging or exit with code -1 if
/// tracing initialization fails. Note that this function may
/// not return.
pub fn init_tracing(verbosity: u8, logfile: Option<&str>, ansi: bool) {
    let Err(err) = init_tracing_inner(verbosity, logfile, ansi) else {
        return;
    };
    fmt().with_max_level(tracing::Level::WARN).init();
    error!("{err}");
    exit(-1);
}

/// This function initializes logging return error it fails to do so
pub fn init_tracing_inner(verbosity: u8, logfile: Option<&str>, ansi: bool) -> Result<(), String> {
    let filter = verbosity_to_filter(verbosity)?;
    let logger = fmt().with_env_filter(filter).with_ansi(ansi);
    if let Some(path) = logfile {
        let file = prepare_log_file(path)?;
        logger.with_writer(file).init();
    } else {
        logger.init();
    }
    Ok(())
}

/// This function converts verbosity to filter or returns error on failure
fn verbosity_to_filter(verbosity: u8) -> Result<EnvFilter, String> {
    let level = match verbosity {
        0 => LevelFilter::WARN,
        1 => LevelFilter::INFO,
        2 => LevelFilter::DEBUG,
        3 => LevelFilter::TRACE,
        _ => return Err("Maximum verbosity level supported is 3".to_owned()),
    };
    let filter = EnvFilter::builder()
        .with_default_directive(level.into())
        .from_env()
        .map_err(|e| format!("Failed to prepare filter: {e}"))?;
    Ok(filter)
}

fn prepare_log_file(path: &str) -> Result<File, String> {
    OpenOptions::new()
        .create_new(true)
        .append(true)
        .open(path)
        .map_err(|e| format!("Failed to open {path}: {e}"))
}