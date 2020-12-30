use clap::{App, Arg, Shell, SubCommand};

use crate::vote::VoteStrategy;

pub(crate) fn build_cli() -> App<'static, 'static> {
    App::new("relaty")
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
                        .value_name("INPUT")
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
                )
                .arg(
                    Arg::with_name("linenumbers")
                        .short("n")
                        .help("Print line numbers"),
                )
                .arg(
                    Arg::with_name("nameonly")
                        .short("N")
                        .help("Only print the entry name"),
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
                .version("0.1.1")
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
            SubCommand::with_name("reset")
                .about("Reset entries to 0/0")
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
            SubCommand::with_name("lock")
                .about("Lock entries")
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
            SubCommand::with_name("unlock")
                .about("Unlock entries")
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
                .version("0.2.0")
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
                        .help("Shows additional information"),
                ),
        )
        .subcommand(
            SubCommand::with_name("completions")
                .about("Generate shell completions")
                .version("0.1.0")
                .author("Lichthagel <lichthagel@tuta.io>")
                .arg(
                    Arg::with_name("shell")
                        .short("s")
                        .value_name("SHELL")
                        .help("Your used shell")
                        .required(true)
                        .takes_value(true)
                        .index(1)
                        .possible_values(&Shell::variants()),
                ),
        )
}
