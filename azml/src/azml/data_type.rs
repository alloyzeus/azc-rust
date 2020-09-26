//

#[derive(Debug)]
pub enum DataType {
    Int8,
    Int16,
    Int32,
    Int64,
    String,
    Struct,
}

impl std::str::FromStr for DataType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int8" => Ok(DataType::Int8),
            "int16" => Ok(DataType::Int16),
            "int32" => Ok(DataType::Int32),
            "int64" => Ok(DataType::Int64),
            "string" => Ok(DataType::String),
            "struct" => Ok(DataType::Struct),
            _ => Err(format!("Unrecognized data type {}", s)),
        }
    }
}
