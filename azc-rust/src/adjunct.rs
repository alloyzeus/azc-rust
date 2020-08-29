//

use crate::{arity, symbol};

#[derive(Debug)]
pub struct Adjunct {
    pub is_entity: bool,
    //TODO: Vec<AdjunctEntity>
    pub entities: Vec<String>,

    pub arity: arity::ArityConstraint,
}

impl symbol::SymbolParameters for Adjunct {}

// struct AdjunctEntity {
//     pub name: String,
// }
