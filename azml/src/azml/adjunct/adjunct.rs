//

use crate::azml::{cardinality, symbol};

// Adjunct can only be hosted by one or more entity-ish objects.
// Adjunct is different from ownership.

//region Adjunct

#[derive(Clone, Debug)]
pub struct Adjunct {
    // TODO: if there are more than one host with the same type,
    // append an alphabet for each of them.
    pub hosts: Vec<AdjunctHost>,

    pub cardinality: cardinality::CardinalityConstraint,

    pub definition: Box<dyn AdjuctDefinition>,

    // Indicates that the symbol name is already prepared. If this is set
    // to false, then the symbol name is assumed non-prepared and the compiler
    // will prepend the names of the hosts to be used as class/struct, model
    // names in the resulting codes.
    pub name_is_prepared: bool,
}

impl symbol::SymbolDefinition for Adjunct {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        self.definition.collect_symbol_refs()
    }
}

//endregion

//region AdjunctDefinition

pub trait AdjuctDefinition:
    mopa::Any + AdjuctDefinitionClone + symbol::SymbolDefinition + std::fmt::Debug
{
}

// Used to implement Clone for AdjunctDefinition
pub trait AdjuctDefinitionClone {
    fn clone_boxed_adjunct_definition(&self) -> Box<dyn AdjuctDefinition>;
}

impl<T> AdjuctDefinitionClone for T
where
    T: AdjuctDefinition + Clone,
{
    fn clone_boxed_adjunct_definition(&self) -> Box<dyn AdjuctDefinition> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn AdjuctDefinition> {
    fn clone(&self) -> Box<dyn AdjuctDefinition> {
        self.clone_boxed_adjunct_definition()
    }
}

mopafy!(AdjuctDefinition);

//endregion

//region AdjunctHost

#[derive(Clone, Debug)]
pub struct AdjunctHost {
    pub kind: symbol::SymbolRef, //TODO: not SymbolRef, but EntityRef

    // The identifier to use. If empty, the compiler
    // will derive from kind field.
    pub name: String,
    //TODO: kind, cardinality, uniqueness
}

//endregion

#[derive(Clone, Debug)]
pub struct AdjunctNone {}

impl AdjuctDefinition for AdjunctNone {}

impl symbol::SymbolDefinition for AdjunctNone {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        Vec::new()
    }
}
