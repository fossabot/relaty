use std::{
    convert::TryFrom,
    io::{self, Write},
};

use crate::{error::Error, rel_vec::RelVec};

pub enum VoteStrategy {
    Random,
    OneMin,
    Equal,
    MinEqual,
}

impl VoteStrategy {
    pub fn strategies() -> [&'static str; 4] {
        ["random", "onemin", "equal", "minequal"]
    }

    pub fn choose_function(&self) -> Box<dyn FnMut(&mut RelVec) -> Option<(usize, usize)>> {
        match self {
            VoteStrategy::Random => Box::new(RelVec::random_pair),
            VoteStrategy::OneMin => Box::new(RelVec::min_pair),
            VoteStrategy::Equal => Box::new(RelVec::equal_pair),
            VoteStrategy::MinEqual => Box::new(RelVec::min_equal_pair),
        }
    }
}

impl ToString for VoteStrategy {
    fn to_string(&self) -> String {
        match self {
            VoteStrategy::Random => "random".to_owned(),
            VoteStrategy::OneMin => "onemin".to_owned(),
            VoteStrategy::Equal => "equal".to_owned(),
            VoteStrategy::MinEqual => "minequal".to_owned(),
        }
    }
}

impl TryFrom<&str> for VoteStrategy {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "random" => Ok(VoteStrategy::Random),
            "onemin" => Ok(VoteStrategy::OneMin),
            "equal" => Ok(VoteStrategy::Equal),
            "minequal" => Ok(VoteStrategy::MinEqual),
            _ => Err(Error::ArgError),
        }
    }
}

pub(crate) fn vote<F: FnMut(&mut RelVec) -> Option<(usize, usize)> + Sized>(
    input: &str,
    output: &str,
    rounds: u32,
    mut choose: F,
    info: bool,
) -> Result<(), Error> {
    let mut rv = RelVec::load(input)?;
    let reader = io::stdin();

    for _ in 0..rounds {
        let (a, b) = match choose(&mut rv) {
            Some((a, b)) => (a, b),
            None => {
                println!("There is no matching pair.");
                return rv.save(output);
            }
        };

        if info {
            println!(
                "(1) {} ({}/{} = {}%)",
                rv[a].name,
                rv[a].wins,
                rv[a].votes,
                rv[a].percentage()
            );
            println!("      vs.");
            println!(
                "(2) {} ({}/{} = {}%)",
                rv[b].name,
                rv[b].wins,
                rv[b].votes,
                rv[b].percentage()
            );
        } else {
            println!("(1) {}", rv[a].name);
            println!("      vs.");
            println!("(2) {}", rv[b].name);
        }
        println!();
        println!("o - Can't decide");
        println!("x - Remove {}", rv[a].name);
        println!("y - Remove {}", rv[b].name);
        println!("q - Quit");
        print!("$ ");

        io::stdout().flush()?;

        let mut cmd = String::new();
        let _s = reader.read_line(&mut cmd)?;

        if let Some(c) = cmd.chars().next() {
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
                (*rv).remove(a);
            } else if c == 'y' {
                (*rv).remove(b);
            } else if c == 'q' {
                return rv.save(output);
            } else {
                println!("unknown command");
            }
        }

        println!("======================");
    }

    rv.save(output)
}
