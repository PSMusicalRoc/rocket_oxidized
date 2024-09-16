pub mod core;

use log::{error, info};
use core::error::RocketErrorTypes;
use core::events::types::quitevent::RocketQuitEventStruct;
use core::events::EVENT_SYSTEM;
use core::RocketApplicationBuilder;
use std::io::BufRead;
use std::process::exit;

fn mainloop(_deltatime: f32) {
    let mut stdin = std::io::stdin().lock();
    let mut s: String = String::new();
    match stdin.read_line(&mut s) {
        Ok(_) => {
            if s.contains("quit") {
                let evsys = EVENT_SYSTEM.write().unwrap();
                let mut quitevent = RocketQuitEventStruct::new();
                evsys.handle_event(&mut quitevent);
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
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
