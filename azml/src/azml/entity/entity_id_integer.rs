//

use crate::azml::symbol;

use crate::azml::{eid, entity::entity_id};

pub type EntityIdInteger = eid::IntegerId;

impl entity_id::EntityIdDefinition for EntityIdInteger {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        self.collect_symbol_refs()
    }
}

pub trait EntityIdIntegerEncoding: mopa::Any + std::fmt::Debug {}

mopafy!(EntityIdIntegerEncoding);
