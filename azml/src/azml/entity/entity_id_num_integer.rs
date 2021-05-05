//

use crate::azml::symbol;

use crate::azml::{entity::entity_id_num, id::id_num};

pub type EntityIdNumInteger = id_num::IntegerIdNum;

impl entity_id_num::EntityIdNumDefinition for EntityIdNumInteger {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        self.collect_symbol_refs()
    }
}

pub trait EntityIdNumIntegerEncoding: mopa::Any + std::fmt::Debug {}

mopafy!(EntityIdNumIntegerEncoding);
