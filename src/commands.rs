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

pub(crate) fn print_screen(
    input: &str,
    filter: Option<&str>,
    linenumbers: bool,
    nameonly: bool,
) -> Result<(), Error> {
    let mut rv = RelVec::load(input)?;
    let re = match filter {
        Some(filter) => Regex::new(filter)?,
        None => Regex::new(".*?")?,
    };

    rv.sort_percentage();
    for (i, e) in rv.iter().enumerate().filter(|(_, e)| re.is_match(&e.name)) {
        if linenumbers {
            print!("{}  ", i + 1);
        }
        if nameonly {
            println!("{}", e.name);
        } else {
            println!("{}", e.to_string());
        }
    }

    Ok(())
}

pub(crate) fn print_file(
    input: &str,
    output: &str,
    filter: Option<&str>,
    linenumbers: bool,
    nameonly: bool,
) -> Result<(), Error> {
    let mut rv = RelVec::load(input)?;
    let output = File::create(output)?;
    let mut writer = BufWriter::new(output);
    let re = match filter {
        Some(filter) => Regex::new(filter)?,
        None => Regex::new(".*?")?,
    };

    rv.sort_percentage();
    for (i, e) in rv.iter().enumerate().filter(|(_, e)| re.is_match(&e.name)) {
        if linenumbers {
            writer.write_all((i + 1).to_string().as_bytes())?;
            writer.write_all(b"  ")?;
        }
        if nameonly {
            writer.write_all(e.name.as_bytes())?;
        } else {
            writer.write_all(e.to_string().as_bytes())?;
        }
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

    for i in rv.iter().filter(|i| re.is_match(&i.name)) {
        println!("{}", i.to_string());
    }

    println!("\x1b[31mDo you want to remove these entries? [y/N]\x1b[0m");
    let mut ans = String::new();
    let reader = io::stdin();
    let _s = reader.read_line(&mut ans)?;

    match ans.chars().next() {
        Some(c) => match c {
            'y' | 'Y' => {
                rv.remove(|i| re.is_match(&i.name));

                rv.save(output)
            }
            _ => {
                println!("Nothing resetted");
                rv.save(output)
            }
        },
        None => {
            println!("Nothing resetted");
            rv.save(output)
        }
    }
}

pub(crate) fn reset(input: &str, output: &str, filter: &str) -> Result<(), Error> {
    let mut rv = RelVec::load(input)?;
    let re = Regex::new(filter)?;

    for i in rv.iter().filter(|i| re.is_match(&i.name)) {
        println!("{}", i.to_string());
    }

    println!("\x1b[31mDo you want to reset these entries? [y/N]\x1b[0m");
    let mut ans = String::new();
    let reader = io::stdin();
    let _s = reader.read_line(&mut ans)?;

    match ans.chars().next() {
        Some(c) => match c {
            'y' | 'Y' => {
                rv.iter_mut()
                    .filter(|i| re.is_match(&i.name))
                    .for_each(|i| i.reset());

                rv.save(output)
            }
            _ => {
                println!("Nothing resetted");
                rv.save(output)
            }
        },
        None => {
            println!("Nothing resetted");
            rv.save(output)
        }
    }
}

pub(crate) fn lock(input: &str, output: &str, filter: &str, lock: bool) -> Result<(), Error> {
    let mut rv = RelVec::load(input)?;
    let re = Regex::new(filter)?;

    for i in rv.iter().filter(|i| re.is_match(&i.name)) {
        println!("{}", i.to_string());
    }

    println!(
        "\x1b[31mDo you want to {}lock these entries? [y/N]\x1b[0m",
        if lock { "" } else { "un" }
    );
    let mut ans = String::new();
    let reader = io::stdin();
    let _s = reader.read_line(&mut ans)?;

    match ans.chars().next() {
        Some(c) => match c {
            'y' | 'Y' => {
                rv.iter_mut()
                    .filter(|i| re.is_match(&i.name))
                    .for_each(|i| i.locked = lock);

                rv.save(output)
            }
            _ => {
                println!("Nothing {}locked", if lock { "" } else { "un" });
                rv.save(output)
            }
        },
        None => {
            println!("Nothing {}locked", if lock { "" } else { "un" });
            rv.save(output)
        }
    }
}

pub(crate) fn stats(input: &str) -> Result<(), Error> {
    // TODO Use one loop
    let rv = RelVec::load(input)?;

    let min_p = rv
        .iter()
        .min_by(|a, b| a.compare_percentage(b))
        .map(|i| i.percentage());
    let max_p = rv
        .iter()
        .max_by(|a, b| a.compare_percentage(b))
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

    println!("Number of entries: \x1b[34m{}\x1b[0m", rv.len());
    println!();

    if let Some(min_p) = min_p {
        println!("Minimum percentage: \x1b[34m{}\x1b[0m", min_p);
    }
    if let Some(max_p) = max_p {
        println!("Maximum percentage: \x1b[34m{}\x1b[0m", max_p);
    }
    for i in &[0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0] {
        let c = rv
            .iter()
            .filter(|e| e.percentage() >= *i && e.percentage() < i + 10.0)
            .count();
        println!(
            "\x1b[33m[{:0>3},{:0>3})\x1b[0m: \x1b[34m{}\x1b[0m \x1b[31m{}\x1b[0m",
            i,
            i + 10.0,
            "|".repeat(c / 5),
            c
        );
    }
    {
        let c = rv
            .iter()
            .filter(|e| e.percentage() >= 90.0 && e.percentage() <= 100.0)
            .count();
        println!(
            "\x1b[33m[090,100]\x1b[0m: \x1b[34m{}\x1b[0m \x1b[31m{}\x1b[0m",
            "|".repeat(c / 5),
            c
        );
    }
    println!();

    if let Some(min_v) = min_v {
        println!("Minimum votes: \x1b[34m{}\x1b[0m", min_v);
    }
    if let Some(max_v) = max_v {
        println!("Maximum votes: \x1b[34m{}\x1b[0m", max_v);
    }
    if let Some(min_v) = min_v {
        if let Some(max_v) = max_v {
            let pad = max_v.to_string().len(); // TODO Improve

            for i in min_v..(max_v + 1) {
                let c = rv.iter().filter(|e| e.votes == i).count();
                println!(
                    "\x1b[33m{}{}\x1b[0m: \x1b[34m{}\x1b[0m \x1b[31m{}\x1b[0m",
                    " ".repeat(pad - i.to_string().len()),
                    i,
                    "|".repeat(c / 5),
                    c
                );
            }
        }
    }
    println!();
    println!("Number of votes: \x1b[34m{}\x1b[0m (est.)", votes / 2);

    Ok(())
}
