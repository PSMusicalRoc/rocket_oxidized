mod logging;

use logging::initialize_logger;
use log::info;

use crate::core::error::RocketError;
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
    initialize_logger();

    info!("Welcome to Rocket {}!", crate::core::ROCKET_VERSION);
    
    // Initialization was successful, return no error
    RocketError::no_error()
}
