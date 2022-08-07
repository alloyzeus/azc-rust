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
            Self::Parsing(ref e) => Some(&**e),
            Self::Internal(ref e) => Some(&**e),
            Self::Msg(_) => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Parsing(ref e) => e.fmt(f),
            Self::Internal(ref e) => e.fmt(f),
            Self::Msg(ref s) => f.write_str(s.as_str()),
        }
    }
}

impl From<io::Error> for Error {
    fn from(x: io::Error) -> Self {
        Self::Internal(Box::new(x))
    }
}

impl From<String> for Error {
    fn from(x: String) -> Self {
        Self::Msg(x)
    }
}

impl From<convert::Infallible> for Error {
    fn from(x: convert::Infallible) -> Self {
        Self::Msg(x.to_string())
    }
}
