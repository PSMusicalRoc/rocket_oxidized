use colored::Colorize;

enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL
}

fn log(
    message:&str,
    level:LogLevel,
    line_number:u32,
    file_name:&str
) {
    match level {
        LogLevel::TRACE => {
            println!("{}\t{}", "[TRACE]".dimmed(), message.dimmed());
        },
        LogLevel::DEBUG => {
            println!("{}\t{}", "[DEBUG]".green().dimmed(), message.green().dimmed());
        },
        LogLevel::INFO => {
            println!("{}\t{}", "[INFO]".green(), message.green());
        },
        LogLevel::WARN => {
            println!("{}",
                format!("[WARN]  Warning in {} at line {}:\n\t{}", file_name, line_number, message).yellow()
            );
        },
        LogLevel::ERROR => {
            println!("{}",
                format!("[ERROR] Error in {} at line {}:\n\t{}", file_name, line_number, message).red()
            );
        },
        LogLevel::FATAL => {
            println!("{}",
                format!("[FATAL] ERROR IN {} AT LINE {}:\n\t{}", file_name, line_number, message).bright_red().underline()
            );
        }
    }
}

pub fn trace(message:&str) {
    log(message, LogLevel::TRACE, 0, "");
}

pub fn debug(message:&str) {
    log(message, LogLevel::DEBUG, 0, "");
}

pub fn info(message:&str) {
    log(message, LogLevel::INFO, 0, "");
}

pub fn warn(message:&str, line_number:u32, file_name:&str) {
    log(message, LogLevel::WARN, line_number, file_name);
}

pub fn error(message:&str, line_number:u32, file_name:&str) {
    log(message, LogLevel::ERROR, line_number, file_name);
}

pub fn fatal(message:&str, line_number:u32, file_name:&str) {
    log(message, LogLevel::FATAL, line_number, file_name);
}

