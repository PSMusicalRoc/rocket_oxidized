use crate::core::events::eventtrait::{RocketEvent, RocketEventTypes};

pub struct RocketQuitEventStruct {

    handled: bool
}

impl RocketQuitEventStruct {

    pub fn new() -> Self {
        RocketQuitEventStruct {
            handled: false
        }
    }
}

impl RocketEvent for  RocketQuitEventStruct {
    fn is_handled(&self) -> bool {
        self.handled
    }

    fn get_event_type(&self) -> RocketEventTypes {
        RocketEventTypes::QuitEvent
    }

    fn handle(&mut self) {
        self.handled = true;
    }
}