//

use crate::azml::{attribute, entity::entity_id, mixin, symbol};

//----

#[derive(Clone, Debug)]
pub struct Entity {
    pub id: entity_id::EntityId,
    pub creation: EntityCreation,
    pub mixins: Vec<mixin::Mixin>,
    pub service: Option<EntityService>,
    pub attributes: Vec<attribute::Attribute>,
}

impl symbol::SymbolDefinition for Entity {}

//----

// Special mixin.
//
// Creation is a special mixin which defines the rule for the creation
// of any instance.
#[derive(Clone, Debug)]
pub struct EntityCreation {
    pub documentation: String,
    pub allow_inter_process: bool,
}

//----

#[derive(Clone, Debug)]
pub struct EntityService {
    pub documentation: String,
    pub enabled: bool,
}
