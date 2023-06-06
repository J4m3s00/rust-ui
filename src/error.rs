use std::fmt::{Debug, Display, Formatter, Result};

pub enum Error {
    Generic(String),
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::Generic(msg) => write!(f, "Generic error: {}", msg),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::Generic(msg) => write!(f, "Generic error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}
