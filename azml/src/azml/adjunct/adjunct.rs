//

use crate::azml::{arity, symbol};

//----

#[derive(Clone, Debug)]
pub struct Adjunct {
    pub hosts: Vec<AdjunctHost>,

    pub arity: arity::ArityConstraint,

    pub definition: Box<dyn AdjuctDefinition>,

    // Indicates that the symbol name is already prepared. If this is set
    // to false, then the symbol name is assumed non-prepared and the compiler
    // will prepend the names of the hosts to be used as class/struct, model
    // names in the resulting codes.
    pub prepared_name: bool,
}

impl symbol::SymbolDefinition for Adjunct {}

//----

pub trait AdjuctDefinition: mopa::Any + AdjuctDefinitionClone + std::fmt::Debug {}

// Used to implement Clone for AdjunctDefinition
pub trait AdjuctDefinitionClone {
    fn clone_box(&self) -> Box<dyn AdjuctDefinition>;
}

impl<T> AdjuctDefinitionClone for T
where
    T: AdjuctDefinition + Clone,
{
    fn clone_box(&self) -> Box<dyn AdjuctDefinition> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn AdjuctDefinition> {
    fn clone(&self) -> Box<dyn AdjuctDefinition> {
        self.clone_box()
    }
}

mopafy!(AdjuctDefinition);

//----

#[derive(Clone, Debug)]
pub struct AdjunctHost {
    pub name: String,
}
