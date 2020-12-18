use std::{
    fs::File,
    io::{BufWriter, Write},
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
    for i in rv.iter().filter(|i| re.is_match(&i.name)) {
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
    for i in rv.iter().filter(|i| re.is_match(&i.name)) {
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

pub(crate) fn stats(input: &str) -> Result<(), Error> {
    let rv = RelVec::load(input)?;

    let min_p = rv
        .iter()
        .min_by(|a, b| b.compare_percentage(a))
        .map(|i| i.percentage());
    let max_p = rv
        .iter()
        .max_by(|a, b| b.compare_percentage(a))
        .map(|i| i.percentage());
    let min_v = rv
        .iter()
        .min_by(|a, b| a.votes.cmp(&b.votes))
        .map(|i| i.votes);
    let max_v = rv
        .iter()
        .max_by(|a, b| a.votes.cmp(&b.votes))
        .map(|i| i.votes);
    let votes: u32 = rv.iter().map(|i| i.votes).sum();

    println!("Number of entries: {}", rv.len());
    println!();

    if let Some(min_p) = min_p {
        println!("Minimum percentage: {}", min_p);
    }
    if let Some(max_p) = max_p {
        println!("Maximum percentage: {}", max_p);
    }
    println!();

    if let Some(min_v) = min_v {
        println!("Minimum votes: {}", min_v);
    }
    if let Some(max_v) = max_v {
        println!("Maximum votes: {}", max_v);
    }
    println!();
    println!("Number of votes: {} (est.)", votes / 2);

    Ok(())
}
