pub mod core;

use log::{error, info};
use core::error::RocketErrorTypes;
use core::RocketApplicationBuilder;

fn mainloop(test: f32) {
    info!("I wanna die {}", test);
}

fn main() {
    
    let mut app = RocketApplicationBuilder::new()
        .set_application_name(format!("Hello World!"))
        .set_mainloop(mainloop)
        .build();

    let status = app.run_application();
    match status.error_code {
        RocketErrorTypes::RocketNoError => {
            info!("Completed execution successfully!");
        },
        _ => {
            error!("{}", status);
        }
    }

}
