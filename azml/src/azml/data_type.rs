//

#[derive(Clone, Debug)]
pub enum DataType {
    Int8,
    Int16,
    Int32,
    Int64,
    String,
    Bytes,
}

impl std::str::FromStr for DataType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int8" => Ok(Self::Int8),
            "int16" => Ok(Self::Int16),
            "int32" => Ok(Self::Int32),
            "int64" => Ok(Self::Int64),
            "string" => Ok(Self::String),
            "bytes" => Ok(Self::Bytes),
            _ => Err(format!("Unrecognized data type {}", s)),
        }
    }
}
