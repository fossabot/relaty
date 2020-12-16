use std::io;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    Bincode(bincode::Error),
    Regex(regex::Error),
    InvalidFileError,
    ArgError,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<bincode::Error> for Error {
    fn from(e: bincode::Error) -> Self {
        Error::Bincode(e)
    }
}

impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        Error::Regex(e)
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::IoError(e) => format!("IOError: {}", e.to_string()),
            Error::Bincode(e) => format!("Serialization Error: {}", e.to_string()),
            Error::Regex(e) => format!("RegEx Error: {}", e.to_string()),
            Error::InvalidFileError => "Not a valid relaty file".to_string(),
            Error::ArgError => "argument is no UTF-8 string".to_string(),
        }
    }
}
