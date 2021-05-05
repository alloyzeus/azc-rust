//

use crate::azml::{abstract_, attribute, mixin, symbol};

use super::{entity_id_num, lifecycle::lifecycle};

//region Entity

#[derive(Clone, Debug)]
pub struct Entity {
    pub id: entity_id_num::EntityId,
    pub implements: abstract_::AbstractImplementation,
    pub lifecycle: lifecycle::Lifecycle,
    pub mixins: Vec<mixin::Mixin>,
    pub service: Option<EntityService>,
    pub attributes: Vec<attribute::Attribute>,
}

impl symbol::SymbolDefinition for Entity {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        let a_syms = self
            .attributes
            .iter()
            .fold(Vec::<symbol::SymbolRef>::new(), |a, b| {
                a.into_iter()
                    .chain(b.collect_symbol_refs())
                    .collect::<Vec<_>>()
            });
        let id_syms = self.id.num.definition.collect_symbol_refs();
        a_syms.into_iter().chain(id_syms.into_iter()).collect()
    }
}

//endregion

//----

#[derive(Clone, Debug)]
pub struct EntityService {
    pub documentation: String,
    pub enabled: bool,
}
