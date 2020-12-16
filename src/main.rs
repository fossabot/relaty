#[macro_use]
extern crate serde_derive;

mod error;
mod rate_vec;

use crate::error::Error;
use clap::{App, Arg, SubCommand};
use fs::File;
use rate_vec::RateVec;
use std::{
    fs,
    io::{BufRead, BufReader, BufWriter, Write},
};

fn main() -> Result<(), Error> {
    let matches = App::new("relaty")
        .version("0.1.0")
        .about("Helps you sort and rate stuff")
        .author("Lichthagel <lichthagel@tuta.io>")
        .subcommand(
            SubCommand::with_name("new")
                .about("Create an empty file")
                .version("0.1.0")
                .author("Lichthagel <lichthagel@tuta.io>")
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

    if let Some(matches) = matches.subcommand_matches("new") {
        return new(matches.value_of("output").ok_or(Error::ArgError)?);
    }

    if let Some(matches) = matches.subcommand_matches("from") {
        return from(
            matches.value_of("input").ok_or(Error::ArgError)?,
            matches.value_of("output").ok_or(Error::ArgError)?,
        );
    }

    if let Some(matches) = matches.subcommand_matches("print") {
        if matches.is_present("output") {
            return print_file(
                matches.value_of("input").ok_or(Error::ArgError)?,
                matches.value_of("output").ok_or(Error::ArgError)?,
            );
        } else {
            return print_screen(matches.value_of("input").ok_or(Error::ArgError)?);
        }
    }

    Ok(())
}

fn new(output: &str) -> Result<(), Error> {
    let rv = RateVec::new();

    rv.save(output)
}

fn from(input: &str, output: &str) -> Result<(), Error> {
    let rv = RateVec::from(input)?;

    rv.save(output)
}

fn print_screen(input: &str) -> Result<(), Error> {
    let rv = RateVec::load(input)?;

    for i in rv.inner.iter() {
        println!("{}", i.to_string());
    }

    Ok(())
}

fn print_file(input: &str, output: &str) -> Result<(), Error> {
    let rv = RateVec::load(input)?;
    let output = File::create(output)?;
    let mut writer = BufWriter::new(output);

    for i in rv.inner.iter() {
        writer.write_all(i.to_string().as_bytes())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}
