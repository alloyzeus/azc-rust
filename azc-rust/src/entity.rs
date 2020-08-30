//

use crate::{mixin, symbol};

#[derive(Debug)]
pub struct Entity {
    pub documentation: String,
    pub service: Option<EntityService>,
    pub mixins: Vec<mixin::Mixin>,
}

impl symbol::SymbolParameters for Entity {}

#[derive(Debug)]
pub struct EntityService {
    pub documentation: String,
}
