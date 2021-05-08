//

use crate::azml::symbol;

use super::entity;

// NOTE: We assume that an abstract is an abstraction for entities. If we
// require a kind of abstraction for other types, we'll discuss on how it
// will be affecting this type.

#[derive(Clone, Debug)]
pub struct Abstract {
    pub documentation: String,

    pub attributes: Vec<AbstractAttribute>,
}

impl symbol::SymbolDefinition for Abstract {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        //TODO: collect from definition
        Vec::new()
    }
}

impl entity::Entity for Abstract {}

// AbstractAttribute defines what attribute must be provided by
// an implementation of an abstract.
#[derive(Clone, Debug)]
pub struct AbstractAttribute {
    pub name: String,
    //TODO: pagination applicable only when the attribute has the type of
    // ordered set.
}

//region AbstractImplementation

#[derive(Clone, Debug)]
pub struct AbstractImplementation {
    pub kind: String,
    pub attributes: Vec<AbstractImplementationAttribute>,
}

#[derive(Clone, Debug)]
pub struct AbstractImplementationAttribute {
    pub identifier: String,
    pub kind: String,
}

//endregion
