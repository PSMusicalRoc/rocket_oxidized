pub mod core;

use core::{error::RocketErrorTypes, startup::initialize_rocket};

fn main() {
    
    let rocket_startup = initialize_rocket();
    match rocket_startup.error_code {
        RocketErrorTypes::RocketNoError => {},
        _ => {
            println!("Rocket failed to start");
            println!("{}", rocket_startup);
        }
    }

}
