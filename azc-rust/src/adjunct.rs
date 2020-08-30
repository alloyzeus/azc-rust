//

use crate::{base::arity, symbol};

#[derive(Debug)]
pub struct Adjunct {
    pub kind: AdjunctKind,

    pub hosts: Vec<AdjuctHost>,

    pub arity: arity::ArityConstraint,
}

impl symbol::SymbolParameters for Adjunct {}

#[derive(Debug)]
pub struct AdjuctHost {
    pub name: String,
}

#[derive(Debug)]
pub enum AdjunctKind {
    ValueObject,
    Entity,
}
