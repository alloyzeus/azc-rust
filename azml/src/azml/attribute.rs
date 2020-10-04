//

use crate::azml::symbol;

#[derive(Clone, Debug)]
pub struct Attribute {
    pub identifier: String,

    pub kind: symbol::SymbolRef,

    // A directive for persistent data immutability. This doesn't affect
    // in-memory data structure.
    pub final_: bool,

    pub identifier_options: AttributeIdentifierOptions,

    pub documentation: String,
}

#[derive(Clone, Debug)]
pub struct AttributeIdentifierOptions {}
