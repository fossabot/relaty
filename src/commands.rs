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
