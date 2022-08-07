//

use std::{
    convert::{self, TryInto},
    result,
};

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
    pub finality: AttributeFinality,

    pub name_options: AttributeNameOptions,

    pub documentation: String,
}

impl Attribute {
    pub fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        vec![self.kind.to_owned()]
    }
}

//endregion

// Only two types of finality: on creation and set once.
//
// For dynamic attributes, i.e., attributes that can be changed anytime,
// user Adjuncts.
#[derive(Clone, Debug)]
pub enum AttributeFinality {
    OnCreation,
    SetOnce,
}

impl convert::TryFrom<String> for AttributeFinality {
    type Error = String;

    fn try_from(s: String) -> result::Result<Self, Self::Error> {
        let sr: &str = s.as_ref();
        sr.try_into()
    }
}

impl convert::TryFrom<&String> for AttributeFinality {
    type Error = String;

    fn try_from(s: &String) -> result::Result<Self, Self::Error> {
        let sr: &str = s.as_ref();
        sr.try_into()
    }
}

impl convert::TryFrom<&str> for AttributeFinality {
    type Error = String;

    fn try_from(s: &str) -> result::Result<Self, Self::Error> {
        match s {
            "on_creation" => Ok(Self::OnCreation),
            "set_once" => Ok(Self::SetOnce),
            _ => Err(format!("Unrecognized AttributeFinality value {}", s).to_owned()),
        }
    }
}

impl From<AttributeFinality> for String {
    fn from(s: AttributeFinality) -> Self {
        (&s).into()
    }
}

impl From<&AttributeFinality> for String {
    fn from(s: &AttributeFinality) -> Self {
        match s {
            AttributeFinality::OnCreation => "on_creation".to_owned(),
            AttributeFinality::SetOnce => "set_once".to_owned(),
        }
    }
}

//region AttributeNameOptions

#[derive(Clone, Debug)]
pub struct AttributeNameOptions {
    //TODO: a list of strategies. pick the best one or twos.
    // - prepared name
    // - scoped name
    // - based on the generators / target languages.
    // - database and other storage
    // - based on cases: IdentifierPascalCase, identifierCamelCase, identifier_snake_case, ...
    // - plural options (focus on English but keep an eye at other cultures)
    pub snake_case: String,
}

//endregion
