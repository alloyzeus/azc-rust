//

use std::{error, fmt, result};

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Error {
    // Use specialized instead of this one
    Internal(Box<dyn error::Error + 'static>),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Internal(ref e) => Some(&**e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Internal(ref e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(x: std::io::Error) -> Error {
        Error::Internal(Box::new(x))
    }
}
