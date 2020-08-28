//

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Error;

impl From<serde_yaml::Error> for Error {
    fn from(_: serde_yaml::Error) -> Error {
        Error
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Error {
        Error
    }
}
