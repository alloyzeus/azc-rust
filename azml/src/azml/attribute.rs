//

use crate::azml::symbol;

//region Attribute

#[derive(Clone, Debug)]
pub struct Attribute {
    pub identifier: String,

    pub kind: symbol::SymbolRef,

    // A directive for persistent data immutability. This doesn't affect
    // in-memory data immutability.
    pub final_: bool,

    pub identifier_options: AttributeIdentifierOptions,

    pub documentation: String,
}

impl Attribute {
    pub fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        vec![self.kind.to_owned()]
    }
}

//endregion

//region AttributeIdentifierOptions

#[derive(Clone, Debug)]
pub struct AttributeIdentifierOptions {
    //TODO: a list of strategies. pick the best one or twos.
// - prepared name
// - scoped name
// - based on the generators / target languages.
// - based on cases: IdentifierPascalCase, identifierCamelCase, identifier_snake_case, ...
// - plural options (focus on English but keep an eye at other cultures)
}

//endregion
