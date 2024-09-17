use crate::core::app::QueryApplication;
use crate::core::events::eventtrait::{RocketEvent, RocketEventTypes};

use log::trace;

pub fn test_quithandler(event: &mut dyn RocketEvent) {
    if !event.is_handled() && event.get_event_type() == RocketEventTypes::QuitEvent {
        trace!("Quit Event called!");
        QueryApplication::quit_application();
        event.handle();
    }
}