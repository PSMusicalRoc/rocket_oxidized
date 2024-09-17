mod appstate;

use appstate::*;
use log::warn;

// ---- USER FACADE FOR INTERACTING WITH THE APPLICATION ----

pub struct QueryApplication {}

impl QueryApplication {

    pub fn is_app_still_running() -> bool {
        let lock = match APPLICATION_STATE.write() {
            Ok(l) => l,
            Err(e) => {
                warn!("Failed to get application state lock: {}", e.to_string());
                return false;
            }
        };
        lock.is_running
    }

    pub fn quit_application() {
        let mut lock = match APPLICATION_STATE.write() {
            Ok(l) => l,
            Err(e) => {
                warn!("Failed to get application state lock: {}", e.to_string());
                return;
            }
        };
        lock.is_running = false;
    }

}
