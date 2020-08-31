//

use std::{error, fmt};

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

impl From<std::io::Error> for Error {
    fn from(x: std::io::Error) -> Error {
        Error::Internal(Box::new(x))
    }
}
