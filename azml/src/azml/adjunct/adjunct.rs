//

use crate::azml::{arity, symbol};

#[derive(Debug)]
pub struct Adjunct {
    pub hosts: Vec<AdjunctHost>,

    pub arity: arity::ArityConstraint,

    pub parameters: Box<dyn AdjuctDefinition>,
}

impl symbol::SymbolDefinition for Adjunct {}

pub trait AdjuctDefinition: mopa::Any + std::fmt::Debug {}

mopafy!(AdjuctDefinition);

#[derive(Debug)]
pub struct AdjunctHost {
    pub name: String,
}
