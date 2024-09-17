//! # Core Library Functions
//! 
//! This file is the root of a solid chunk of the engine,
//! including:
//! - The app builder itself and startup procedures
//! - Windowing and backends
//! - Error system
//! - Event system
//! - App update (and @todo fixedupdate?) loops
//! 
//! To start, include `rocket_oxidized::core::RocketApplicationBuilder`
//! in your project. This struct acts as the creation point
//! for any apps you make with the engine. You are able to
//! provide custom loop code, application name, and
//! other (@todo!) TBD features. To see more, see the
//! page for the [RocketApplicationBuilder] struct itself.
//! 
//! 

use std::time::{Duration, Instant};

use app::QueryApplication;
use error::{RocketError, RocketErrorTypes};
use startup::initialize_rocket;

pub mod app;
pub mod error;
pub mod events;
pub mod startup;

/// The current version of the Rocket Engine! Used
/// primarily for initialization.
pub const ROCKET_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), env!("ROCKET_VERSION_SUFFIX"));

/// The function signature for the "update"
/// function passed into [RocketApplicationBuilder].
pub type RocketMainloopType = fn(f32);

/// Builder for a [RocketApplication]
/// 
/// This struct dictates the creation of a [RocketApplication].
/// It serves to allow the user to customize most everything
/// about the application, including its name, loop functions,
/// and more (@todo!)
/// 
/// Here's a very basic example of using this builder in your
/// code:
/// ```
/// use rocket_oxidized::core::RocketApplicationBuilder;
/// use rocket_oxidized::core::app::QueryApplication;
/// 
/// fn main() {
///     let mut app = RocketApplicationBuilder::new()
///         .set_application_name(format!("Hello world!"))
///         .set_mainloop(|f: f32| {
///             use std::process::exit;
///             exit(0);
///     }).build();
///     app.run_application();
/// }
/// ```
pub struct RocketApplicationBuilder {
    /// The name of the built application
    application_name: String,

    /// The function that will be called in the
    /// 'update' thread (aka, not fixed deltatime)
    mainloop: RocketMainloopType
}

impl RocketApplicationBuilder {

    /// Creates a new Builder. Call this in your
    /// `main.rs` file to start creating your application.
    pub fn new() -> RocketApplicationBuilder {
        RocketApplicationBuilder {
            application_name: format!(""),
            mainloop: |_| {}
        }
    }

    /// Sets the application name for the created
    /// application. This is used primarily in the
    /// window's titlebar.
    /// 
    /// # Parameters
    /// - `name`: The specific name of the application.
    pub fn set_application_name(&mut self, name: String) -> &mut Self {
        self.application_name = name.clone();
        self
    }

    /// Provides the "update" loop for the created
    /// application. This loop is deltatime-dependent,
    /// and runs as fast as possible. Good for input
    /// grabbing, less good for physics and rendering.
    /// 
    /// # Parameters
    /// - `mainloop`: A function (either lambda or otherwise)
    /// that matches the [RocketMainloopType] signature.
    pub fn set_mainloop(&mut self, mainloop: RocketMainloopType) -> &mut Self {
        self.mainloop = mainloop;
        self
    }

    /// Builds and returns a [RocketApplication] based
    /// on the builder inputs. From here, the user can
    /// run their program.
    pub fn build(&mut self) -> RocketApplication {
        RocketApplication {
            application_name: self.application_name.clone(),
            mainloop: self.mainloop
        }
    }
}

/// A runnable application
///
/// The actual application object returned by the
/// [RocketApplicationBuilder::build()] method. This
/// struct allows the user to run their custom-built
/// program through the [RocketApplication::run_application()]
/// method.
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
