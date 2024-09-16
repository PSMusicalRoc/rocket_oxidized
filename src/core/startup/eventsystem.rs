use log::{info, trace};

use crate::core::events::eventtrait::{RocketEvent, RocketEventTypes};
use crate::core::events::EVENT_SYSTEM;
use crate::core::error::RocketError;
use crate::core::app::{APPLICATION_STATE, appstate::AppState};

pub fn test_quithandler(event: &mut dyn RocketEvent) {
    if !event.is_handled() && event.get_event_type() == RocketEventTypes::QuitEvent {
        trace!("Quit Event called!");
        let mut lock = APPLICATION_STATE.write().unwrap();
        lock.quit_application();
        event.handle();
    }
}

pub fn initialize_event_system() -> RocketError {

    info!("Initializing EventSystem...");
    // acquire eventsystem lock
    let mut eventsystem_lock = EVENT_SYSTEM.write().unwrap();
    trace!("EventSystem lock obtained!");

    eventsystem_lock.add_handler(test_quithandler);
    trace!("Added QuitHandler!");


    trace!("Done instantiating EventSystem!");
    RocketError::no_error()
}