//

use crate::symbol;

#[derive(Debug)]
pub struct Adjunct {
    pub is_entity: bool,
    pub entities: Vec<String>,
}

impl symbol::SymbolParameters for Adjunct {}
