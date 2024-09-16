use std::sync::RwLock;

use lazy_static::lazy_static;

pub mod appstate;
use appstate::*;

lazy_static!{

    pub static ref APPLICATION_STATE: RwLock<AppStateContainer> = RwLock::new(AppStateContainer::new());

}