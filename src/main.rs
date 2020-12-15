#[macro_use]
extern crate serde_derive;

mod error;
mod rate_vec;

use std::{fs, io::{BufRead, BufReader, BufWriter, Write}};

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
        .subcommand(
            SubCommand::with_name("print")
                .about("Print a file to screen or to a file")
                .version("0.1.0")
                .author("Lichthagle <lichthagel@tuta.io>")
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
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("from") {
        from(&matches);
        return;
    }

    if let Some(matches) = matches.subcommand_matches("print") {
        if matches.is_present("output") {
            print_file(matches.value_of("input").unwrap(), matches.value_of("output").unwrap())
        } else {
            print_screen(matches.value_of("input").unwrap())
        }
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

fn print_screen(input: &str) {
    let rv = RateVec::load(input).unwrap();

    for i in rv.inner.iter() {
        println!("{}", i.to_string());
    }
}

fn print_file(input: &str, output: &str) {
    let rv = RateVec::load(input).unwrap();
    let output = File::create(output).unwrap();
    let mut writer = BufWriter::new(output);

    for i in rv.inner.iter() {
        writer.write_all(i.to_string().as_bytes()).unwrap();
        writer.write_all("\n".as_bytes()).unwrap();
    }
}
