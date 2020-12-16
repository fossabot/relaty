#[macro_use]
extern crate serde_derive;

mod error;
mod rel_vec;

use crate::error::Error;
use clap::{App, Arg, SubCommand};
use fs::File;
use rel_vec::RelVec;
use std::{
    fs,
    io::{BufWriter, Write},
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
                )
                .arg(
                    Arg::with_name("item")
                        .short("i")
                        .value_name("ITEM")
                        .help("Insert item")
                        .takes_value(true)
                        .multiple(true),
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
        if let Some(items) = matches.values_of("item") {
            let items = items.collect();

            return create(matches.value_of("output").ok_or(Error::ArgError)?, items);
        } else {
            return new(matches.value_of("output").ok_or(Error::ArgError)?);
        }
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
    let rv = RelVec::new();

    rv.save(output)
}

fn create(output: &str, items: Vec<&str>) -> Result<(), Error> {
    let rv = RelVec::create(items.into_iter().map(|i| i.to_owned()).collect());

    rv.save(output)
}

fn from(input: &str, output: &str) -> Result<(), Error> {
    let rv = RelVec::from(input)?;

    rv.save(output)
}

fn print_screen(input: &str) -> Result<(), Error> {
    let rv = RelVec::load(input)?;

    for i in rv.inner.iter() {
        println!("{}", i.to_string());
    }

    Ok(())
}

fn print_file(input: &str, output: &str) -> Result<(), Error> {
    let rv = RelVec::load(input)?;
    let output = File::create(output)?;
    let mut writer = BufWriter::new(output);

    for i in rv.inner.iter() {
        writer.write_all(i.to_string().as_bytes())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}
