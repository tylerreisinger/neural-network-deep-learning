use std::io;
use std::fmt;
use std::result;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum MnistError {
    Io(io::Error),
    InvalidFormat,
    Parse(),
}

pub type Result<T> = result::Result<T, MnistError>;

impl From<io::Error> for MnistError {
    fn from(error: io::Error) -> MnistError {
        MnistError::Io(error)
    }
}

impl Display for MnistError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MnistError::Io(ref err) => write!(f, "{}", err),
            MnistError::InvalidFormat => 
                write!(f, "{}", self.description()),
            MnistError::Parse() =>
                write!(f, "Parse error"),
        }
    }
}

impl Error for MnistError {
    fn description(&self) -> &str {
        match *self {
            MnistError::Io(ref err) => err.description(),
            MnistError::InvalidFormat => "Invalid format",
            MnistError::Parse() => "Unable to parse",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            MnistError::Io(ref err) => Some(err),
            MnistError::InvalidFormat => None,
            MnistError::Parse() => None,
        }
    }
}

