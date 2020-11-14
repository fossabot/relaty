use crate::error::Error;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RateEntry {
    name: String,
    wins: u32,
    votes: u32,
}

impl RateEntry {
    pub fn new(name: String, wins: u32, votes: u32) -> RateEntry {
        RateEntry { name, wins, votes }
    }
}

impl PartialEq for RateEntry {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for RateEntry {}

impl PartialEq<RateEntry> for &mut RateEntry {
    fn eq(&self, other: &RateEntry) -> bool {
        self.name == other.name
    }
}

pub struct RateVec {
    inner: Vec<RateEntry>,
    rng: ThreadRng,
}

impl RateVec {
    fn new() -> Self {
        Self {
            inner: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    fn create(names: Vec<String>) -> Self {
        Self {
            inner: names.into_iter().map(|s| RateEntry::new(s, 0, 0)).collect(),
            rng: rand::thread_rng(),
        }
    }

    fn from<P: AsRef<Path>>(file: P) -> Result<Self, io::Error> {
        let f = File::open(file)?;
        let reader = BufReader::new(f);

        Ok(Self {
            inner: reader
                .lines()
                .map(|r| r.map(|s| RateEntry::new(s, 0, 0)))
                .collect::<Result<Vec<RateEntry>, io::Error>>()?,
            rng: rand::thread_rng(),
        })
    }

    fn load<P: AsRef<Path>>(file: P) -> Result<Self, Error> {
        let f = File::open(file)?;
        let reader = BufReader::new(f);

        Ok(Self {
            inner: bincode::deserialize_from(reader)?,
            rng: rand::thread_rng(),
        })
    }

    fn save<P: AsRef<Path>>(&self, file: P) -> Result<(), Error> {
        let f = File::open(file)?;
        let writer = BufWriter::new(f);

        bincode::serialize_into(writer, &self.inner)?;
        Ok(())
    }

    // could be improved maybe
    fn random_pair(&mut self) -> Option<(&mut RateEntry, &mut RateEntry)> {
        if self.inner.len() < 2 {
            return None;
        }

        let i1 = self.rng.gen_range(0, self.inner.len());
        let mut i2 = self.rng.gen_range(0, self.inner.len() - 1);
        if i2 >= i1 {
            i2 += 1;
        }

        let mut iter = self.inner.iter_mut();

        if i1 < i2 {
            let item1 = iter.nth(i1).unwrap();
            let item2 = iter.nth(i2 - i1 - 1).unwrap();
            Some((item1, item2))
        } else {
            let item1 = iter.nth(i2).unwrap();
            let item2 = iter.nth(i1 - i2 - 1).unwrap();
            Some((item1, item2))
        }
    }
}
