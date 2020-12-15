#[macro_use]
extern crate serde_derive;

mod error;
mod rate_vec;

use std::{fs, io::{BufRead, BufReader}};

use clap::{App, Arg, ArgMatches, SubCommand};
use fs::File;
use rate_vec::RateVec;

fn main() {
    let matches = App::new("relaty")
        .version("0.1.0")
        .about("Helps you sort and rate stuff")
        .author("Lichthagel <lichthagel@tuta.io>")
        .subcommand(
            SubCommand::with_name("from")
                .about("Create data from a text file")
                .version("0.1.0")
                .author("Lichthagel <lichthagel@tuta.io>")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .value_name("FILE")
                        .help("Input file")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .value_name("FILE")
                        .help("Output file")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("from") {
        from(&matches);
        return;
    }
}

fn from(matches: &ArgMatches) {
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();

    let input = File::open(input).unwrap();
    let bufreader = BufReader::new(input);

    let rv = RateVec::create(bufreader.lines().map(|s| s.unwrap()).collect());

    rv.save(output).unwrap();
}
