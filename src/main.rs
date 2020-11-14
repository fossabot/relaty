#[macro_use]
extern crate serde_derive;

mod error;
mod rate_vec;

use clap::App;

fn main() {
    App::new("relrate")
        .version("0.1.0")
        .about("Helps you rate stuff")
        .author("Lichthagel")
        .get_matches();
}
