use crate::core::app::APPLICATION_STATE;
use crate::core::app::appstate::AppState;
use crate::core::events::eventtrait::{RocketEvent, RocketEventTypes};

use log::trace;

pub fn test_quithandler(event: &mut dyn RocketEvent) {
    if !event.is_handled() && event.get_event_type() == RocketEventTypes::QuitEvent {
        trace!("Quit Event called!");
        let mut lock = APPLICATION_STATE.write().unwrap();
        lock.quit_application();
        event.handle();
    }
}