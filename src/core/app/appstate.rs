use lazy_static::lazy_static;
use std::sync::RwLock;

// pub trait AppState {

//     fn is_app_still_running(&self) -> bool;
//     fn quit_application(&mut self);
// }

trait RocketResettable {

    fn reset(&mut self);
}

pub struct AppStateContainer {

    pub is_running: bool
}

impl AppStateContainer {

    pub fn new() -> Self {
        AppStateContainer { is_running: true }
    }
}

impl RocketResettable for AppStateContainer {

    fn reset(&mut self) {
        self.is_running = true;
    }
}


// impl AppState for AppStateContainer {

//     fn is_app_still_running(&self) -> bool {
//         self.is_running
//     }

//     fn quit_application(&mut self) {
//         self.is_running = false;
//     }
// }

unsafe impl Send for AppStateContainer {}
unsafe impl Sync for AppStateContainer {}


lazy_static!{

    pub static ref APPLICATION_STATE: RwLock<AppStateContainer> = RwLock::new(AppStateContainer::new());

}