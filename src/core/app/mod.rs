//! Functionality regarding the overall
//! state of the application.
//! 
//! This module defines two things:
//! 1) Internal state that can be queried and
//! updated by users, and
//! 2) A facade for users to access such that
//! they can query and update internal state.

mod appstate;

use appstate::*;
use log::warn;

// ---- USER FACADE FOR INTERACTING WITH THE APPLICATION ----

/// The user facade for interfacing with
/// the application's internal state
/// 
/// This class will contain all user actions
/// that can be taken on internal engine state,
/// both in querying that state as well as
/// mutating it.
pub struct QueryApplication {}

impl QueryApplication {

    /// Checks to see if the application is
    /// still considered "running".
    /// 
    /// # Returns
    /// `true` if the app is running, `false`
    /// otherwise.
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

    /// Sets the app's internal state such that
    /// it is considered no longer running. This
    /// state will be reflected in
    /// [QueryApplication::is_app_still_running()].
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
