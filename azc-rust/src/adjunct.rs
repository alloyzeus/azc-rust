//

use crate::symbol;

#[derive(Debug)]
pub struct Adjunct {
    pub is_entity: bool,
    //TODO: Vec<AdjunctEntity>
    pub entities: Vec<String>,
}

impl symbol::SymbolParameters for Adjunct {}

// struct AdjunctEntity {
//     pub name: String,
// }
