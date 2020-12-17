use std::{
    convert::TryFrom,
    io::{self, Write},
};

use crate::{error::Error, rel_vec::RelVec};

pub enum VoteStrategy {
    Random,
    OneMin,
    Equal,
}

impl VoteStrategy {
    pub fn strategies() -> [&'static str; 3] {
        ["random", "onemin", "equal"]
    }

    pub fn choose_function(&self) -> Box<dyn FnMut(&mut RelVec) -> Option<(usize, usize)>> {
        match self {
            VoteStrategy::Random => Box::new(RelVec::random_pair),
            VoteStrategy::OneMin => Box::new(RelVec::min_pair),
            VoteStrategy::Equal => Box::new(RelVec::equal_pair)
        }
    }
}

impl ToString for VoteStrategy {
    fn to_string(&self) -> String {
        match self {
            VoteStrategy::Random => "random".to_owned(),
            VoteStrategy::OneMin => "onemin".to_owned(),
            VoteStrategy::Equal => "equal".to_owned()
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
            _ => Err(Error::ArgError),
        }
    }
}

pub(crate) fn vote<F: FnMut(&mut RelVec) -> Option<(usize, usize)> + Sized>(
    input: &str,
    output: &str,
    rounds: u32,
    mut choose: F,
) -> Result<(), Error> {
    let mut rv = RelVec::load(input)?;
    let reader = io::stdin();

    for _ in 0..rounds {
        let (a, b) = choose(&mut rv).ok_or(Error::ArgError)?;

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
