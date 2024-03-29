//

use crate::azml::symbol;

//region ValueObject

#[derive(Clone, Debug)]
pub struct ValueObject {
    pub definition: Box<dyn ValueObjectDefinition>,
}

impl symbol::SymbolDefinition for ValueObject {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        //TODO: collect from definition
        Vec::new()
    }
}

//endregion

//----

pub trait ValueObjectDefinition: mopa::Any + ValueObjectDefinitionClone + std::fmt::Debug {}

// Used to implement Clone for AdjunctDefinition
pub trait ValueObjectDefinitionClone {
    fn clone_boxed_value_object_definition(&self) -> Box<dyn ValueObjectDefinition>;
}

impl<T> ValueObjectDefinitionClone for T
where
    T: ValueObjectDefinition + Clone,
{
    fn clone_boxed_value_object_definition(&self) -> Box<dyn ValueObjectDefinition> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn ValueObjectDefinition> {
    fn clone(&self) -> Box<dyn ValueObjectDefinition> {
        self.clone_boxed_value_object_definition()
    }
}

mopafy!(ValueObjectDefinition);

//----

// A ValueObjectAlias is used to alias other type. The other type could be
// one of primitive types (string, int8, int16, int32, int64, float32, float64, blob)
// or a named struct.
#[derive(Clone, Debug)]
pub struct ValueObjectAlias {
    pub data_type: symbol::SymbolRef,
}

impl ValueObjectDefinition for ValueObjectAlias {}

//----

#[derive(Clone, Debug)]
pub struct ValueObjectStruct {
    pub key: Option<ValueObjectStructKey>,
    pub fields: Vec<ValueObjectStructField>,
}

impl ValueObjectDefinition for ValueObjectStruct {}

#[derive(Clone, Debug)]
pub struct ValueObjectStructKey {
    pub fields: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ValueObjectStructField {
    pub identifier: String,
    pub data_type: symbol::SymbolRef,
}
