use std::{io, num::ParseIntError};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    Serde(serde_json::Error),
    Regex(regex::Error),
    Parse(ParseIntError),
    ArgError,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        Error::Regex(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::Parse(e)
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::IoError(e) => format!("IOError: {}", e.to_string()),
            Error::Serde(e) => format!("Serialization Error: {}", e.to_string()),
            Error::Regex(e) => format!("RegEx Error: {}", e.to_string()),
            Error::Parse(e) => format!("Parse Error: {}", e.to_string()),
            Error::ArgError => "argument is no UTF-8 string".to_string(),
        }
    }
}
