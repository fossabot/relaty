use crate::error::Error;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RateEntry {
    pub name: String,
    pub wins: u32,
    pub votes: u32,
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

impl ToString for RateEntry {
    fn to_string(&self) -> String {
        format!(
            "{} - {}/{} - {}%",
            self.name,
            self.wins,
            self.votes,
            f64::from(self.wins) * 100.0 / f64::from(self.votes)
        )
    }
}

#[derive(Clone, Debug)]
pub struct RateVec {
    pub inner: Vec<RateEntry>,
    rng: ThreadRng,
}

impl RateVec {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn create(names: Vec<String>) -> Self {
        Self {
            inner: names.into_iter().map(|s| RateEntry::new(s, 0, 0)).collect(),
            rng: rand::thread_rng(),
        }
    }

    pub fn from<P: AsRef<Path>>(file: P) -> Result<Self, io::Error> {
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

    pub fn load<P: AsRef<Path>>(file: P) -> Result<Self, Error> {
        let f = File::open(file)?;
        let reader = BufReader::new(f);

        Ok(Self {
            inner: bincode::deserialize_from(reader)?,
            rng: rand::thread_rng(),
        })
    }

    pub fn save<P: AsRef<Path>>(&self, file: P) -> Result<(), Error> {
        let f = File::create(file)?;
        let writer = BufWriter::new(f);

        bincode::serialize_into(writer, &self.inner)?;
        Ok(())
    }

    pub fn min_votes(&mut self) -> Vec<&mut RateEntry> {
        let mut min = u32::max_value();
        let mut v = Vec::new();

        for item in &mut self.inner {
            match item.votes.cmp(&min) {
                Ordering::Less => {
                    min = item.votes;
                    v = Vec::new();
                    v.push(item);
                }
                Ordering::Equal => {
                    v.push(item);
                }
                Ordering::Greater => {}
            }
        }

        v
    }

    pub fn random_pair(&mut self) -> Option<(&mut RateEntry, &mut RateEntry)> {
        if self.inner.len() < 2 {
            return None;
        }

        let i1 = self.rng.gen_range(0, self.inner.len());
        let mut i2 = self.rng.gen_range(0, self.inner.len() - 1);
        if i2 >= i1 {
            i2 += 1;
        }

        if i1 < i2 {
            let (a, b) = self.inner.split_at_mut(i2);

            Some((&mut a[i1], &mut b[0]))
        } else {
            let (a, b) = self.inner.split_at_mut(i1);

            Some((&mut b[0], &mut a[i2]))
        }
    }
}

impl PartialEq for RateVec {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::{BufWriter, Write},
    };

    use super::{RateEntry, RateVec};

    #[test]
    fn rate_entry_new() {
        assert_eq!(
            RateEntry {
                name: "abc".to_owned(),
                wins: 125132,
                votes: 12551
            },
            RateEntry::new("abc".to_owned(), 125132, 12551)
        )
    }

    #[test]
    fn rate_entry_partial_eq() {
        let a = RateEntry {
            name: "abc".to_owned(),
            wins: 125132,
            votes: 1263,
        };
        let b = RateEntry {
            name: "abc".to_owned(),
            wins: 1251,
            votes: 1361621,
        };

        assert_eq!(a, b);
    }

    #[test]
    fn rate_entry_to_string() {
        let a = RateEntry {
            name: "abc".to_owned(),
            wins: 12,
            votes: 36,
        };

        assert_eq!(a.to_string(), "abc - 12/36 - 33.333333333333336%");
    }

    #[test]
    fn rate_vec_new() {
        let a = RateVec {
            inner: Vec::new(),
            rng: rand::thread_rng(),
        };
        let b = RateVec::new();

        assert_eq!(a, b);
    }

    #[test]
    fn rate_vec_create() {
        let a = RateVec {
            inner: Vec::from([
                RateEntry::new("abc".to_string(), 1251, 16162),
                RateEntry::new("adsga".to_string(), 1251, 1236),
            ]),
            rng: rand::thread_rng(),
        };
        let b = RateVec::create(Vec::from(["abc".to_string(), "adsga".to_string()]));

        assert_eq!(a, b);
    }

    #[test]
    fn rate_vec_from() {
        let file = File::create("_rate_vec_from.txt").unwrap();
        let mut writer = BufWriter::new(file);

        writer.write_all(b"a\nb\nc\n").unwrap();

        drop(writer);

        let a = RateVec::create(["a".to_string(), "b".to_string(), "c".to_string()].to_vec());
        let b = RateVec::from("_rate_vec_from.txt").unwrap();

        fs::remove_file("_rate_vec_from.txt").unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn rate_vec_load() {
        let file = File::create("_rate_vec_load.txt").unwrap();
        let mut writer = BufWriter::new(file);

        writer
            .write_all(&[
                0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x61, 0x62, 0x63, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ])
            .unwrap();

        drop(writer);

        let a = RateVec::create(["abc".to_string()].to_vec());
        let b = RateVec::load("_rate_vec_load.txt").unwrap();

        fs::remove_file("_rate_vec_load.txt").unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn rate_vec_save() {
        let rv = RateVec::create(["abc".to_string()].to_vec());
        rv.save("_rate_vec_save.txt").unwrap();

        let a: [u8; 27] = [
            0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x61,
            0x62, 0x63, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ];
        let b = fs::read("_rate_vec_save.txt").unwrap();

        fs::remove_file("_rate_vec_save.txt").unwrap();

        assert_eq!(&a, b.as_slice());
    }

    #[test]
    fn rate_vec_min_votes() {
        let mut rv = RateVec {
            inner: [
                RateEntry {
                    name: "abc".to_string(),
                    wins: 12,
                    votes: 123,
                },
                RateEntry {
                    name: "bcd".to_string(),
                    wins: 125,
                    votes: 123,
                },
                RateEntry {
                    name: "cde".to_string(),
                    wins: 12,
                    votes: 12632,
                },
            ]
            .to_vec(),
            rng: rand::thread_rng(),
        };

        assert_eq!(
            rv.min_votes(),
            Vec::from([
                &mut RateEntry {
                    name: "abc".to_string(),
                    wins: 12,
                    votes: 123,
                },
                &mut RateEntry {
                    name: "bcd".to_string(),
                    wins: 125,
                    votes: 123
                }
            ])
        );
    }

    #[test]
    fn rate_vec_random_pair() {
        let mut rv = RateVec {
            inner: [
                RateEntry {
                    name: "abc".to_string(),
                    wins: 0,
                    votes: 0,
                },
                RateEntry {
                    name: "def".to_string(),
                    wins: 0,
                    votes: 0,
                },
            ]
            .to_vec(),
            rng: rand::thread_rng(),
        };

        let (a, b) = rv.random_pair().unwrap();
        let (c, d) = (
            &mut RateEntry {
                name: "abc".to_string(),
                wins: 0,
                votes: 0,
            },
            &mut RateEntry {
                name: "def".to_string(),
                wins: 0,
                votes: 0,
            },
        );

        assert!((&a, &b) == (&c, &d) || (&a, &b) == (&d, &c));
    }
}
