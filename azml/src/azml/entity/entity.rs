//

use crate::azml::{attribute, entity::entity_id, mixin, symbol};

//region Entity

#[derive(Clone, Debug)]
pub struct Entity {
    pub id: entity_id::EntityId,
    pub creation: EntityCreation,
    pub mixins: Vec<mixin::Mixin>,
    pub service: Option<EntityService>,
    pub attributes: Vec<attribute::Attribute>,
}

impl symbol::SymbolDefinition for Entity {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        self.attributes
            .iter()
            .fold(Vec::<symbol::SymbolRef>::new(), |a, b| {
                a.into_iter()
                    .chain(b.collect_symbol_refs())
                    .collect::<Vec<_>>()
            })
    }
}

//endregion

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
