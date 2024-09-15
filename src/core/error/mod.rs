use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum RocketErrorTypes {
    RocketNoError = 0,
    RocketFailedToInitLogger
}

impl Display for RocketErrorTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(dead_code)]
pub struct RocketError {
    pub error_message: String,
    pub error_code: RocketErrorTypes
}

impl Display for RocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error_code, self.error_message)
    }
}

