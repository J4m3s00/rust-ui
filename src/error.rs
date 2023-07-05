use std::fmt::{Debug, Display, Formatter};

pub enum Error {
    Generic(String),
    IO(std::io::Error),
    SvgParseError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Generic(msg) => write!(f, "Generic error: {}", msg),
            Error::IO(err) => write!(f, "IO error: {}", err),
            Error::SvgParseError(msg) => write!(f, "Error parsing SVG: {}", msg),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Generic(msg) => write!(f, "Generic error: {}", msg),
            Error::IO(err) => write!(f, "IO error: {}", err),
            Error::SvgParseError(msg) => write!(f, "Error parsing SVG: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Error::SvgParseError(_) | Error::Generic(_) => None,
            Error::IO(err) => Some(err),
        }
    }
}

impl<T> From<T> for Error
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Error::Generic(value.into())
    }
}
