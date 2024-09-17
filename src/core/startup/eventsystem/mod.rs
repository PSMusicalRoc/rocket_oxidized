mod handlers;

use log::{info, trace};

use crate::core::events::EVENT_SYSTEM;
use crate::core::error::RocketError;

pub fn initialize_event_system() -> RocketError {

    info!("Initializing EventSystem...");
    // acquire eventsystem lock
    let mut eventsystem_lock = EVENT_SYSTEM.write().unwrap();
    trace!("EventSystem lock obtained!");

    eventsystem_lock.add_handler(handlers::quithandler::test_quithandler);
    trace!("Added QuitHandler!");


    trace!("Done instantiating EventSystem!");
    RocketError::no_error()
}