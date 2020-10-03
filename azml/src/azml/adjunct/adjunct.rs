//

use crate::azml::{arity, symbol};

//----

#[derive(Clone, Debug)]
pub struct Adjunct {
    pub hosts: Vec<AdjunctHost>,

    pub arity: arity::ArityConstraint,

    pub definition: Box<dyn AdjuctDefinition>,

    // A flag to indicate the generators to not prepend host names for
    // the resulting class/struct, model names.
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
