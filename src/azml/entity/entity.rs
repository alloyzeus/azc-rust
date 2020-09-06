//

use crate::azml::{entity::entity_id, mixin, symbol};

#[derive(Debug)]
pub struct Entity {
    pub documentation: String,
    pub id: entity_id::EntityId,
    pub service: Option<EntityService>,
    pub mixins: Vec<mixin::Mixin>,
}

impl symbol::SymbolDefinition for Entity {}

#[derive(Debug)]
pub struct EntityService {
    pub documentation: String,
    pub enabled: bool,
}
