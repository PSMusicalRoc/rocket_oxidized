pub mod eventsystem;
pub mod eventtrait;
pub mod types;

use std::sync::RwLock;
use eventsystem::RocketEventSystem;

use eventtrait::RocketEvent;
use lazy_static::lazy_static;

use crate::core::error::{RocketError, RocketErrorTypes};

lazy_static!{
    pub static ref EVENT_SYSTEM: RwLock<RocketEventSystem> = RwLock::new(RocketEventSystem::new());
}


// ---- EVENT SYSTEM FUNCTIONS ----

pub fn send_event(event: &mut impl RocketEvent) -> RocketError {

    let evsys = match EVENT_SYSTEM.write() {
        Ok(es) => es,
        Err(e) => {
            return RocketError {
                error_code: RocketErrorTypes::RocketFailedToAcquireLockError,
                error_message: e.to_string()
            }
        }
    };

    evsys.handle_event(event);

    RocketError::no_error()
}

