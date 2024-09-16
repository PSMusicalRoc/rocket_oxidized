use error::{RocketError, RocketErrorTypes};
use startup::initialize_rocket;

pub mod backends;
pub mod error;
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

        loop {
            (self.mainloop)(1.0);
            break;
        }
        RocketError::no_error()
    }
}
