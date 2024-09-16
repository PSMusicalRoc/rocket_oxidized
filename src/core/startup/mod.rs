mod eventsystem;
mod logging;

use eventsystem::initialize_event_system;
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
    let logger_setup = initialize_logger();
    match logger_setup.error_code {
        RocketErrorTypes::RocketNoError => {},
        _ => { return logger_setup; }
    }

    info!("Welcome to Rocket {}!", crate::core::ROCKET_VERSION);
    
    let eventsystem_setup = initialize_event_system();
    match eventsystem_setup.error_code {
        RocketErrorTypes::RocketNoError => {},
        _ => { return eventsystem_setup; }
    }

    // Initialization was successful, return no error
    RocketError::no_error()
}
