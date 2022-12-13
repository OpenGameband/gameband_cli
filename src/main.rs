extern crate core;

use oxidefurnace::gameband::open_gameband;
use oxidefurnace::time::set_time;

fn main() {
    let gameband = open_gameband();
    match gameband {
        None => {
            println!("Gameband not found");
        }
        Some(gb) => {
            set_time(gb).expect("Failed to set time");
        }
    }
}
