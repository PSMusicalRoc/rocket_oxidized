pub mod core;

use log::{error, info};
use core::error::RocketErrorTypes;
use core::RocketApplicationBuilder;
use std::process::exit;

fn mainloop(deltatime: f32) {
    println!("Deltatime: {}", deltatime);
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
            exit(0);
        },
        _ => {
            error!("{}", status);
            exit(1);
        }
    }

}
