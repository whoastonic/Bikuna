use std::error::Error;
use std::fmt::{self, Debug, Display};

#[derive(Debug)]
pub enum BikunaError {
    Connection(String),
    FileSys(String),
    Writer(String),
    Reader(String),
    Input(String),
}

pub type BikunaResult<T> = Result<T, BikunaError>;

impl Display for BikunaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BikunaError::Connection(err) => write!(f, "ERR [ CONNECTION ] - ({})", err),
            BikunaError::FileSys(err) => write!(f, "ERR [ READER ] - ({})", err),
            BikunaError::Reader(err) => write!(f, "ERR [ READER ] - ({})", err),
            BikunaError::Writer(err) => write!(f, "ERR [ READER ] - ({})", err),
            BikunaError::Input(err) => write!(f, "ERR [ INPUT ] - ({})", err),
        }
    }
}

impl Error for BikunaError {}
