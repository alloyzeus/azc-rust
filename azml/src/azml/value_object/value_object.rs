//

use crate::azml::{data_type, symbol};

#[derive(Debug)]
pub struct ValueObject {
    pub documentation: String,

    pub definition: Box<dyn ValueObjectDefinition>,
}

impl symbol::SymbolDefinition for ValueObject {}

pub trait ValueObjectDefinition: mopa::Any + std::fmt::Debug {}

mopafy!(ValueObjectDefinition);

#[derive(Debug)]
pub struct ValueObjectPrimitive {
    pub documentation: String,

    pub data_type: data_type::DataType,
}

impl ValueObjectDefinition for ValueObjectPrimitive {}
