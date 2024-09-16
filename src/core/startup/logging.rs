use env_logger;
use colored::Colorize;
use std::io::Write;
use std::panic::catch_unwind;
use crate::core::error::{RocketError, RocketErrorTypes};

/// Initializes the logger with colored stdout
/// and filenames/linenumbers.
#[cfg(not(debug_assertions))]
pub fn initialize_logger() -> RocketError {

    let result = catch_unwind(|| {
        env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .format(|buf, record| {
                
                let fmt_str = format!("[{}]: {}",
                    record.level(),
                    record.args());

                writeln!(buf, "{}",
                    match record.level() {
                        log::Level::Trace => {
                            fmt_str.white()
                        },
                        log::Level::Debug => {
                            fmt_str.cyan()
                        }
                        log::Level::Info => {
                            fmt_str.green()
                        },
                        log::Level::Warn => {
                            fmt_str.yellow()
                        },
                        log::Level::Error => {
                            fmt_str.red()
                        }
                    }
                )
            })
            .init();
    });

    if result.is_err() {
        return RocketError {
            error_code: RocketErrorTypes::RocketInitLoggerError,
            error_message: format!("Failed to initialize logger. Did you already create an application?")
        };
    }

    RocketError::no_error()
}

#[cfg(debug_assertions)]
pub fn initialize_logger() -> RocketError {
    use log::trace;

    let result = catch_unwind(|| {
        env_logger::builder()
            .filter_level(log::LevelFilter::Trace)
            .format(|buf, record| {
                
                let fmt_str = format!("[{}->{}] [{}]: {}",
                    record.file().unwrap_or("<nofile>"),
                    record.line().unwrap_or(0),
                    record.level(),
                    record.args());

                writeln!(buf, "{}",
                    match record.level() {
                        log::Level::Trace => {
                            fmt_str.white()
                        },
                        log::Level::Debug => {
                            fmt_str.cyan()
                        }
                        log::Level::Info => {
                            fmt_str.green()
                        },
                        log::Level::Warn => {
                            fmt_str.yellow()
                        },
                        log::Level::Error => {
                            fmt_str.red()
                        }
                    }
                )
            })
            .init();
    });

    if result.is_err() {
        return RocketError {
            error_code: RocketErrorTypes::RocketInitLoggerError,
            error_message: format!("Failed to initialize logger. Did you already create an application?")
        };
    }

    trace!("Created logger!");

    RocketError::no_error()
}
