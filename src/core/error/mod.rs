//! # Error Handling
//! 
//! Rocket uses an error struct to do its error
//! handling. It contains two properties:
//! - An enum Error Code that give information about
//! what type of error occurred
//! - A string message for detailed information about
//! why the error occurred
//! 
//! Many Rocket functions will return an error struct,
//! and the user should check if that error struct's code
//! matches "RocketNoError", which signifies an OK return.


use std::fmt::Display;

/// The errorcodes used in Rocket. `RocketNoError` signifies
/// a lack of an error, while any other code means that
/// something bad occurred.
#[derive(Debug, PartialEq, Eq)]
pub enum RocketErrorTypes {
    /// There is no error to report.
    RocketNoError = 0,

    /// There was an error initializing the logger
    /// Rocket uses, most likely having to do with
    /// creating multiple RocketApplications at once.
    RocketInitLoggerError,

    /// Something went wrong with the chosen graphical
    /// backend.
    RocketBackendError,
    
    /// Some lock that Rocket tried to acquire (usually
    /// for global state) was unable to be acquired.
    RocketFailedToAcquireLockError
}

impl Display for RocketErrorTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// The Rocket Engine error struct. This contains
/// both the error code of whatever was being thrown,
/// as well as the message about why the error was
/// thrown.
#[allow(dead_code)]
pub struct RocketError {
    /// The string message detailing why the error was
    /// thrown
    pub error_message: String,

    /// The errorcode enum - details the type of error that
    /// was thrown
    pub error_code: RocketErrorTypes
}

impl RocketError {

    /// Shorthand for creating a RocketError contianing
    /// the code `RocketNoError` and the message `No Error`.
    pub fn no_error() -> RocketError {
        RocketError {
            error_code: RocketErrorTypes::RocketNoError,
            error_message: format!("No Error")
        }
    }
}

impl Display for RocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error_code, self.error_message)
    }
}

