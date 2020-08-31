//

pub use serde_yaml::{from_reader, from_value, Value};

pub use crate::base::error::Error;

impl From<serde_yaml::Error> for Error {
    fn from(x: serde_yaml::Error) -> Error {
        Error::Parsing(Box::new(x))
    }
}
