//

use crate::azml::{id::ref_key, symbol};

#[derive(Clone, Debug)]
pub struct EntityId {
    pub num: EntityIdNum,
    pub ref_key: ref_key::RefKey,
}

//region EntityIdNum

#[derive(Clone, Debug)]
pub struct EntityIdNum {
    pub definition: Box<dyn EntityIdNumDefinition>,
}

//endregion

//region EntityIdNumDefinition

pub trait EntityIdNumDefinition: mopa::Any + EntityIdNumDefinitionClone + std::fmt::Debug {
    //NOTE: should simply add symbol::SymbolDefinition but we have some
    // conflict for the clone_box.
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef>;
}

mopafy!(EntityIdNumDefinition);

pub trait EntityIdNumDefinitionClone {
    fn clone_box(&self) -> Box<dyn EntityIdNumDefinition>;
}

impl<T> EntityIdNumDefinitionClone for T
where
    T: EntityIdNumDefinition + Clone,
{
    fn clone_box(&self) -> Box<dyn EntityIdNumDefinition> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn EntityIdNumDefinition> {
    fn clone(&self) -> Box<dyn EntityIdNumDefinition> {
        self.clone_box()
    }
}

//endregion
