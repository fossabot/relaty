use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use clap::Values;
use regex::Regex;

use crate::{error::Error, rel_vec::RelVec};

pub(crate) fn new(output: &str) -> Result<(), Error> {
    let rv = RelVec::new();

    rv.save(output)
}

pub(crate) fn create(output: &str, items: Vec<&str>) -> Result<(), Error> {
    let rv = RelVec::create(items.into_iter().map(|i| i.to_owned()).collect());

    rv.save(output)
}

pub(crate) fn from(input: &str, output: &str) -> Result<(), Error> {
    let rv = RelVec::from(input)?;

    rv.save(output)
}

pub(crate) fn print_screen(input: &str, filter: Option<&str>) -> Result<(), Error> {
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

pub(crate) fn print_file(input: &str, output: &str, filter: Option<&str>) -> Result<(), Error> {
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

pub(crate) fn add(input: &str, output: &str, items: Values) -> Result<(), Error> {
    let mut rv = RelVec::load(input)?;

    for i in items {
        rv.add(i.to_owned());
    }

    rv.save(output)
}

pub(crate) fn remove(input: &str, output: &str, filter: &str) -> Result<(), Error> {
    let mut rv = RelVec::load(input)?;
    let re = Regex::new(filter)?;

    rv.remove(|i| re.is_match(&i.name));

    rv.save(output)
}

pub(crate) fn vote(input: &str, output: &str, rounds: u32) -> Result<(), Error> {
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
