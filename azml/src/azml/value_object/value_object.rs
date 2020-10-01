//

use crate::azml::{data_type, symbol};

#[derive(Clone, Debug)]
pub struct ValueObject {
    pub documentation: String,

    //TODO: should be two kinds: alias (of a primitive or other value_object)
    // or struct (composite).
    pub data_type: data_type::DataType,

    // Required only when data_type is Struct
    pub struct_: Option<ValueObjectStruct>,
}

impl symbol::SymbolDefinition for ValueObject {}

// A ValueObjectAlias is used to alias other type. The other type could be
// one of primitive types (string, int8, int16, int32, int64, float32, float64, blob)
// or a named struct.
#[derive(Clone, Debug)]
pub struct ValueObjectAlias {
    pub documentation: String,
}

#[derive(Clone, Debug)]
pub struct ValueObjectStruct {
    pub documentation: String,
}
