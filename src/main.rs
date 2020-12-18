#[macro_use]
extern crate serde_derive;

mod commands;
mod error;
mod rel_vec;
mod vote;

use std::convert::TryInto;

use crate::commands::{add, create, new, remove, stats};
use crate::error::Error;
use crate::vote::{vote, VoteStrategy};
use clap::{App, Arg, SubCommand};
use commands::{from, print_file, print_screen};

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
            SubCommand::with_name("stats")
                .about("Show stats about a list")
                .version("0.1.0")
                .author("Lichthagel <lichthagel@tuta.io>")
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .value_name("FIILE")
                        .help("List file")
                        .required(true)
                        .takes_value(true)
                        .index(1),
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
                )
                .arg(
                    Arg::with_name("strategy")
                        .short("s")
                        .value_name("STRATEGY")
                        .help("Strategy to use")
                        .takes_value(true)
                        .default_value("random")
                        .possible_values(&VoteStrategy::strategies()),
                )
                .arg(
                    Arg::with_name("info")
                    .short("i")
                    .help("Shows additional information")
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

    if let Some(matches) = matches.subcommand_matches("stats") {
        let input = matches.value_of("file").ok_or(Error::ArgError)?;

        return stats(input);
    }

    if let Some(matches) = matches.subcommand_matches("vote") {
        let input = matches.value_of("file").ok_or(Error::ArgError)?;
        let output = matches.value_of("output").unwrap_or(input);
        let rounds = matches
            .value_of("rounds")
            .ok_or(Error::ArgError)?
            .parse::<u32>()?;
        let strategy: VoteStrategy = matches
            .value_of("strategy")
            .ok_or(Error::ArgError)?
            .try_into()?;
        let info = matches.is_present("info");

        println!("Using strategy \"{}\"", strategy.to_string());
        return vote(input, output, rounds, strategy.choose_function(), info);
    }

    Ok(())
}
