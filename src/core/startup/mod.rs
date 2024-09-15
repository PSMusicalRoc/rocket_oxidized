mod logging;

use logging::initialize_logger;
use log::info;

use crate::core::error::{RocketError, RocketErrorTypes};
/// Initializes the Rocket Engine:
/// - Starts the logger
/// 
/// # Returns
/// 
/// A `RocketError` struct containing either
/// the error and a descriptive message, or
/// a `RocketError` struct whose error code
/// is `RocketNoError`, indicating success.
pub fn initialize_rocket() -> RocketError {

    // intitialize logging
    let log_status = initialize_logger();
    match log_status.error_code {
        RocketErrorTypes::RocketNoError => {}
        _ => { return log_status; }
    }

    info!("Welcome to Rocket!");
    
    // Initialization was successful, return no error
    RocketError {
        error_code: RocketErrorTypes::RocketNoError,
        error_message: format!("No Error")
    }
}
