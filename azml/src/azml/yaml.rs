//

pub use serde_yaml::{from_reader, from_value, Value};

pub use crate::azml::error::Error;

impl From<serde_yaml::Error> for Error {
    fn from(x: serde_yaml::Error) -> Self {
        Self::Parsing(Box::new(x))
    }
}
