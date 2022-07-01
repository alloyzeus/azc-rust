//

use crate::azml::symbol;

// Independent attributes, proxy/projection attributes (related concept: view in SQL)
// context-derived attributes (finals).
//
// Attribute modifiers:
// - final: must be provided at creation, the attribute is permanent. in
//   implementation, we create a column for the attribute. example:
//   verification status. if the verification is revoked, the entry must be
//   set as deleted.
// - dynamic: can be changed anytime. in implementation, this requires a
//   side table for each kind of attribute. active value for an attribute is
//   indicated by a flag.
// - dynamic-limited: can be changed if still within quota.

//region Attribute

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,

    pub kind: symbol::SymbolRef,

    // A directive for persistent data immutability. This doesn't affect
    // in-memory data immutability.
    pub final_: bool,

    pub name_options: AttributeNameOptions,

    pub documentation: String,
}

impl Attribute {
    pub fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        vec![self.kind.to_owned()]
    }
}

//endregion

//region AttributeNameOptions

#[derive(Clone, Debug)]
pub struct AttributeNameOptions {
    //TODO: a list of strategies. pick the best one or twos.
    // - prepared name
    // - scoped name
    // - based on the generators / target languages.
    // - based on cases: IdentifierPascalCase, identifierCamelCase, identifier_snake_case, ...
    // - plural options (focus on English but keep an eye at other cultures)
}

//endregion
