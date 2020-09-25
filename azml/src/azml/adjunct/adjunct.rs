//

use crate::azml::{arity, symbol};

#[derive(Debug)]
pub struct Adjunct {
    pub hosts: Vec<AdjunctHost>,

    pub arity: arity::ArityConstraint,

    pub parameters: Box<dyn AdjuctDefinition>,
}

pub trait AdjuctDefinition: mopa::Any + std::fmt::Debug {}

mopafy!(AdjuctDefinition);

impl symbol::SymbolDefinition for Adjunct {}

#[derive(Debug)]
pub struct AdjunctHost {
    pub name: String,
}
