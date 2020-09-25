//

use crate::azml::symbol;

#[derive(Debug)]
pub struct ValueObject {
    pub documentation: String,
}

impl symbol::SymbolDefinition for ValueObject {}
