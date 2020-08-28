//

use crate::module;

#[derive(Debug)]
pub struct Entity {
    pub description: String,
    pub service: Option<EntityService>,
}

impl module::SymbolParameters for Entity {}

#[derive(Debug)]
pub struct EntityService {
    pub description: String,
}
