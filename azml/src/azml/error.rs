//

use std::{convert, error, fmt, io};

#[derive(Debug)]
pub enum Error {
    Parsing(Box<dyn error::Error + 'static>),

    // Use specialized instead of this one
    Internal(Box<dyn error::Error + 'static>),
    Msg(String),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Parsing(ref e) => Some(&**e),
            Error::Internal(ref e) => Some(&**e),
            Error::Msg(_) => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Parsing(ref e) => e.fmt(f),
            Error::Internal(ref e) => e.fmt(f),
            Error::Msg(ref s) => f.write_str(s.as_str()),
        }
    }
}

impl From<io::Error> for Error {
    fn from(x: io::Error) -> Error {
        Error::Internal(Box::new(x))
    }
}

impl From<String> for Error {
    fn from(x: String) -> Error {
        Error::Msg(x)
    }
}

impl From<convert::Infallible> for Error {
    fn from(x: convert::Infallible) -> Error {
        Error::Msg(x.to_string())
    }
}
