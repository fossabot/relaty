#[macro_use]
extern crate serde_derive;

mod error;
mod rel_vec;

use crate::error::Error;
use clap::{App, Arg, SubCommand, Values};
use fs::File;
use regex::Regex;
use rel_vec::RelVec;
use std::{
    fs,
    io::{self, BufWriter, Write},
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
                        .value_name("OUTPUT")
                        .help("Output file")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("item")
                        .short("i")
                        .value_name("ITEM")
                        .help("Insert item")
                        .takes_value(true)
                        .multiple(true)
                        .index(2),
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
                        .value_name("OUTPUT")
                        .help("Input file")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .value_name("OUTPUT")
                        .help("Output file")
                        .required(true)
                        .takes_value(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("print")
                .about("Print a file to screen or to a file")
                .version("0.1.0")
                .author("Lichthagle <lichthagel@tuta.io>")
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .value_name("FILE")
                        .help("List file")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .value_name("OUTPUT")
                        .help("Output file")
                        .takes_value(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("filter")
                        .short("f")
                        .value_name("filter")
                        .help("Filter items by name")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Add elements to a storage file")
                .version("0.1.0")
                .author("Lichthagel <lichthagel@tuta.io>")
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .value_name("FILE")
                        .help("List file")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .value_name("OUTPUT")
                        .help("Output file")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("item")
                        .value_name("ITEM")
                        .help("Item to add")
                        .multiple(true)
                        .takes_value(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("Remove elements from a list")
                .version("0.1.0")
                .author("Lichthagel <lichthagel@tuta.io>")
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .value_name("FILE")
                        .help("List file")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .value_name("OUTPUT")
                        .help("Output file")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("filter")
                        .value_name("FILTER")
                        .help("Filter")
                        .required(true)
                        .takes_value(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("vote")
                .about("Vote several times")
                .version("0.1.0")
                .author("Lichthagel <lichthagel@tuta.io>")
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .value_name("FILE")
                        .help("List file")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .value_name("OUTPUT")
                        .help("Output file")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("rounds")
                        .short("r")
                        .value_name("ROUNDS")
                        .help("Number of rounds")
                        .takes_value(true)
                        .index(2)
                        .default_value("10"),
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
                matches.value_of("file").ok_or(Error::ArgError)?,
                matches.value_of("output").ok_or(Error::ArgError)?,
                matches.value_of("filter"),
            );
        } else {
            return print_screen(
                matches.value_of("file").ok_or(Error::ArgError)?,
                matches.value_of("filter"),
            );
        }
    }

    if let Some(matches) = matches.subcommand_matches("add") {
        let input = matches.value_of("file").ok_or(Error::ArgError)?;
        let output = matches.value_of("output").unwrap_or(input);

        if let Some(items) = matches.values_of("item") {
            return add(input, output, items);
        }
    }

    if let Some(matches) = matches.subcommand_matches("remove") {
        let input = matches.value_of("file").ok_or(Error::ArgError)?;
        let output = matches.value_of("output").unwrap_or(input);
        let filter = matches.value_of("filter").ok_or(Error::ArgError)?;

        return remove(input, output, filter);
    }

    if let Some(matches) = matches.subcommand_matches("vote") {
        let input = matches.value_of("file").ok_or(Error::ArgError)?;
        let output = matches.value_of("output").unwrap_or(input);
        let rounds = matches
            .value_of("rounds")
            .ok_or(Error::ArgError)?
            .parse::<u32>()?;

        return vote(input, output, rounds);
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

fn print_screen(input: &str, filter: Option<&str>) -> Result<(), Error> {
    let mut rv = RelVec::load(input)?;
    let re = match filter {
        Some(filter) => Regex::new(filter)?,
        None => Regex::new(".*?")?,
    };

    rv.sort_percentage();
    for i in rv.inner.iter().filter(|i| re.is_match(&i.name)) {
        println!("{}", i.to_string());
    }

    Ok(())
}

fn print_file(input: &str, output: &str, filter: Option<&str>) -> Result<(), Error> {
    let mut rv = RelVec::load(input)?;
    let output = File::create(output)?;
    let mut writer = BufWriter::new(output);
    let re = match filter {
        Some(filter) => Regex::new(filter)?,
        None => Regex::new(".*?")?,
    };

    rv.sort_percentage();
    for i in rv.inner.iter().filter(|i| re.is_match(&i.name)) {
        writer.write_all(i.to_string().as_bytes())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}

fn add(input: &str, output: &str, items: Values) -> Result<(), Error> {
    let mut rv = RelVec::load(input)?;

    for i in items {
        rv.add(i.to_owned());
    }

    rv.save(output)
}

fn remove(input: &str, output: &str, filter: &str) -> Result<(), Error> {
    let mut rv = RelVec::load(input)?;
    let re = Regex::new(filter)?;

    rv.remove(|i| re.is_match(&i.name));

    rv.save(output)
}

fn vote(input: &str, output: &str, rounds: u32) -> Result<(), Error> {
    let mut rv = RelVec::load(input)?;
    let reader = io::stdin();

    for _ in 0..rounds {
        let (a, b) = rv.random_pair().ok_or(Error::ArgError)?;

        println!("{} vs. {}", rv[a].name, rv[b].name);
        println!("");
        println!("1 - Vote for {}", rv[a].name);
        println!("2 - Vote for {}", rv[b].name);
        println!("o - Can't decide");
        println!("x - Remove {}", rv[a].name);
        println!("y - Remove {}", rv[b].name);
        print!("$ ");

        io::stdout().flush()?;

        let mut cmd = String::new();
        let _s = reader.read_line(&mut cmd)?;

        match cmd.chars().next() {
            Some(c) => {
                if c == '1' {
                    rv[a].wins += 1;
                    rv[a].votes += 1;
                    rv[b].votes += 1;
                } else if c == '2' {
                    rv[b].wins += 1;
                    rv[a].votes += 1;
                    rv[b].votes += 1;
                } else if c == 'o' {
                } else if c == 'x' {
                    rv.inner.remove(a);
                } else if c == 'y' {
                    rv.inner.remove(b);
                } else {
                    println!("unknown command");
                }
            }
            None => {}
        }

        println!("======================");
    }

    rv.save(output)
}
