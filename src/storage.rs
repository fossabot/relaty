use crate::{entry_struct::RateEntry, error::Error};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter},
    path::Path,
};

fn read_lines<P: AsRef<Path>>(file: P) -> Result<Vec<String>, io::Error> {
    let f = File::open(file)?;
    let reader = BufReader::new(f);

    reader.lines().collect()
}

fn create_data(names: Vec<String>) -> Vec<RateEntry> {
    names.into_iter().map(|s| RateEntry::new(s, 0, 0)).collect()
}

fn load_data<P: AsRef<Path>>(file: P) -> Result<Vec<RateEntry>, Error> {
    let f = File::open(file)?;
    let reader = BufReader::new(f);

    Ok(bincode::deserialize_from(reader)?)
}

fn save_data<P: AsRef<Path>>(file: P, data: &[RateEntry]) -> Result<(), Error> {
    let f = File::open(file)?;
    let writer = BufWriter::new(f);

    bincode::serialize_into(writer, data)?;
    Ok(())
}
