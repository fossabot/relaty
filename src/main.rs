#[macro_use]
extern crate serde_derive;

mod cli;
mod commands;
mod error;
mod rel_vec;
mod vote;

use std::{convert::TryInto, io, str::FromStr};

use crate::commands::{add, create, new, remove, reset, stats};
use crate::error::Error;
use crate::vote::{vote, VoteStrategy};
use clap::Shell;
use commands::{from, lock, print_file, print_screen};

fn main() -> Result<(), Error> {
    let matches = crate::cli::build_cli().get_matches();

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
                matches.is_present("linenumbers"),
                matches.is_present("nameonly"),
            );
        } else {
            return print_screen(
                matches.value_of("file").ok_or(Error::ArgError)?,
                matches.value_of("filter"),
                matches.is_present("linenumbers"),
                matches.is_present("nameonly"),
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

    if let Some(matches) = matches.subcommand_matches("reset") {
        let input = matches.value_of("file").ok_or(Error::ArgError)?;
        let output = matches.value_of("output").unwrap_or(input);
        let filter = matches.value_of("filter").ok_or(Error::ArgError)?;

        return reset(input, output, filter);
    }

    if let Some(matches) = matches.subcommand_matches("lock") {
        let input = matches.value_of("file").ok_or(Error::ArgError)?;
        let output = matches.value_of("output").unwrap_or(input);
        let filter = matches.value_of("filter").ok_or(Error::ArgError)?;

        return lock(input, output, filter, true);
    }

    if let Some(matches) = matches.subcommand_matches("unlock") {
        let input = matches.value_of("file").ok_or(Error::ArgError)?;
        let output = matches.value_of("output").unwrap_or(input);
        let filter = matches.value_of("filter").ok_or(Error::ArgError)?;

        return lock(input, output, filter, false);
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

    if let Some(matches) = matches.subcommand_matches("completions") {
        let shell = matches.value_of("shell").ok_or(Error::ArgError)?;

        cli::build_cli().gen_completions_to(
            "relaty",
            Shell::from_str(shell).map_err(|_| Error::ArgError)?,
            &mut io::stdout(),
        );

        return Ok(());
    }

    cli::build_cli()
        .print_long_help()
        .map_err(|_| Error::ArgError)
}
