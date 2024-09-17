use std::time::{Duration, Instant};

use app::QueryApplication;
use error::{RocketError, RocketErrorTypes};
use startup::initialize_rocket;

pub mod app;
pub mod error;
pub mod events;
pub mod startup;

pub const ROCKET_VERSION: &str = "v0.0.0 dev";

pub type RocketMainloopType = fn(f32);

pub struct RocketApplicationBuilder {
    application_name: String,
    mainloop: RocketMainloopType
}

impl RocketApplicationBuilder {

    pub fn new() -> RocketApplicationBuilder {
        RocketApplicationBuilder {
            application_name: format!(""),
            mainloop: |_| {}
        }
    }

    pub fn set_application_name(&mut self, name: String) -> &mut Self {
        self.application_name = name.clone();
        self
    }

    pub fn set_mainloop(&mut self, mainloop: RocketMainloopType) -> &mut Self {
        self.mainloop = mainloop;
        self
    }

    pub fn build(&mut self) -> RocketApplication {
        RocketApplication {
            application_name: self.application_name.clone(),
            mainloop: self.mainloop
        }
    }
}

#[allow(dead_code)]
pub struct RocketApplication {
    application_name: String,
    mainloop: fn(f32)
}

impl RocketApplication {

    pub fn run_application(&mut self) -> RocketError {

        let rocket_startup = initialize_rocket();
        match rocket_startup.error_code {
            RocketErrorTypes::RocketNoError => {},
            _ => { return rocket_startup; }
        }

        let mut time_point: Instant = Instant::now();

        'update: loop {
            if !QueryApplication::is_app_still_running() {
                break 'update;
            }
            
            let tmp_time_point = Instant::now();
            let deltatime: Duration = time_point.elapsed();
            time_point = tmp_time_point;

            // @todo maybe return an error?
            (self.mainloop)(deltatime.as_secs_f32());
        }
        RocketError::no_error()
    }
}
