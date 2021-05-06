//

use crate::azml::symbol;

// Entities are objects that have identity.

//region Entity

pub trait Entity: symbol::SymbolDefinition {}

//endregion

//----

#[derive(Clone, Debug)]
pub struct EntityService {
    pub documentation: String,
    pub enabled: bool,
}
