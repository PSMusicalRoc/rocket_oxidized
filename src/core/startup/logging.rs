use env_logger;
use colored::Colorize;
use std::io::Write;
use crate::core::error::{RocketError, RocketErrorTypes};

/// Initializes the logger with colored stdout
/// and filenames/linenumbers.
pub fn initialize_logger() -> RocketError {

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

    RocketError {
        error_code: RocketErrorTypes::RocketNoError,
        error_message: format!("No Error")
    }
}