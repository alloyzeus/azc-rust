//

use crate::{arity, symbol};

#[derive(Debug)]
pub struct Adjunct {
    pub is_entity: bool,

    pub entities: Vec<AdjunctEntity>,

    pub arity: arity::ArityConstraint,
}

impl symbol::SymbolParameters for Adjunct {}

#[derive(Debug)]
pub struct AdjunctEntity {
    pub name: String,
}
