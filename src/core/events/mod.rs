pub mod eventsystem;
pub mod eventtrait;
pub mod types;

use std::sync::RwLock;
use eventsystem::RocketEventSystem;

use lazy_static::lazy_static;

lazy_static!{
    pub static ref EVENT_SYSTEM: RwLock<RocketEventSystem> = RwLock::new(RocketEventSystem::new());
}


