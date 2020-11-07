mod entry_struct;

use crate::entry_struct::RateEntry;
use clap::App;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() {
    App::new("relrate")
        .version("0.1.0")
        .about("Helps you rate stuff")
        .author("Lichthagel")
        .get_matches();
}

fn read_lines<P: AsRef<Path>>(file: P) -> Result<Vec<String>, io::Error> {
    let f = File::open(file)?;
    let reader = BufReader::new(f);

    reader.lines().collect()
}

fn read_data<P: AsRef<Path>>(file: P) -> Result<Vec<RateEntry>, io::Error> {
    read_lines(file).map(|v| v.into_iter().map(|s| RateEntry::new(s, 0, 0)).collect())
}
