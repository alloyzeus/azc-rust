//

use crate::azml::{primitive, symbol};

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

    pub type_name: primitive::PrimitiveType,
}

impl ValueObjectDefinition for ValueObjectPrimitive {}
