//

use crate::symbol;

#[derive(Debug)]
pub struct Entity {
    pub description: String,
    pub service: Option<EntityService>,
}

impl symbol::SymbolParameters for Entity {}

#[derive(Debug)]
pub struct EntityService {
    pub description: String,
}
