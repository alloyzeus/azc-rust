//

use crate::azml::{data_type, symbol};

#[derive(Debug)]
pub struct ValueObject {
    pub documentation: String,

    pub data_type: data_type::DataType,

    // Required only when data_type is Struct
    pub struct_: Option<ValueObjectStruct>,
}

impl symbol::SymbolDefinition for ValueObject {}

#[derive(Debug)]
pub struct ValueObjectStruct {
    pub documentation: String,
}
