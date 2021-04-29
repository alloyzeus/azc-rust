//

use std::{
    convert::{self, TryInto},
    result,
};

use crate::azml::{abstract_, adjunct::adjunct, attribute, id_num, ref_key, symbol};

//region AdjunctEntity

//TODO: cardinality

#[derive(Clone, Debug)]
pub struct AdjunctEntity {
    pub ordering: AdjunctEntityOrdering,
    pub id_num: AdjunctEntityIdNum,
    pub ref_key: ref_key::RefKey,
    pub implements: abstract_::AbstractImplementation,
    // This affects RefKey structure.
    pub scope: AdjunctEntityScope,
    pub attributes: Vec<attribute::Attribute>,
}

impl adjunct::AdjuctDefinition for AdjunctEntity {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        let a_syms = self
            .attributes
            .iter()
            .fold(Vec::<symbol::SymbolRef>::new(), |a, b| {
                a.into_iter()
                    .chain(b.collect_symbol_refs())
                    .collect::<Vec<_>>()
            });
        let id_syms = self.id_num.definition.collect_symbol_refs();
        a_syms.into_iter().chain(id_syms.into_iter()).collect()
    }
}

//endregion

//region AdjunctEntiyScope

// This is used to determine whether an instance can be addressed directly
// or that it requires going through its hosts.
//
// Some example of adjuncts with global scope are shop items
// in a marketplace. Some marketplace systems provide URLs which refer
// to the items directly without giving information which store these
// items belong to. It shows, e.g., https://example.com/items/12345678
// instead of https://example.com/stores/345/items/12345678
//
// The Global scope requires the ordering to be Unordered.
//
//TODO: find a better terms as what we have here now will create confusions for federated system.
#[derive(Clone, PartialEq, Debug)]
pub enum AdjunctEntityScope {
    Local,

    // An adjunct entity with global scope will make it more similar to
    // Entity. It's still an adjunct of other entity but an instance
    // is directly addressable instead of through its entities.
    // A global adjunct entity can only have unordered ordering. Its
    // instances' IDs are random and globally unique.
    Global,
}

impl Default for AdjunctEntityScope {
    fn default() -> AdjunctEntityScope {
        AdjunctEntityScope::Local
    }
}

impl convert::TryFrom<String> for AdjunctEntityScope {
    type Error = String;

    fn try_from(s: String) -> result::Result<Self, Self::Error> {
        let sr: &str = s.as_ref();
        sr.try_into()
    }
}

impl convert::TryFrom<&str> for AdjunctEntityScope {
    type Error = String;

    fn try_from(s: &str) -> result::Result<Self, Self::Error> {
        match s {
            "local" | "" => Ok(AdjunctEntityScope::Local),
            "global" => Ok(AdjunctEntityScope::Global),
            _ => Err(format!("Unrecognized AdjunctEntityScope value {}", s).to_owned()),
        }
    }
}

//endregion

//region AdjunctEntityIdNum

#[derive(Clone, Debug)]
pub struct AdjunctEntityIdNum {
    pub definition: Box<dyn AdjunctEntityIdNumDefinition>,
}

//endregion

//region AdjunctEntityIdNumDefinition

pub trait AdjunctEntityIdNumDefinition:
    mopa::Any + AdjunctEntityIdNumDefinitionClone + std::fmt::Debug
{
    //NOTE: should simply add symbol::SymbolDefinition but we have some
    // conflict for the clone_box.
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef>;
}

mopafy!(AdjunctEntityIdNumDefinition);

pub trait AdjunctEntityIdNumDefinitionClone {
    fn clone_box(&self) -> Box<dyn AdjunctEntityIdNumDefinition>;
}

impl<T> AdjunctEntityIdNumDefinitionClone for T
where
    T: AdjunctEntityIdNumDefinition + Clone,
{
    fn clone_box(&self) -> Box<dyn AdjunctEntityIdNumDefinition> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn AdjunctEntityIdNumDefinition> {
    fn clone(&self) -> Box<dyn AdjunctEntityIdNumDefinition> {
        self.clone_box()
    }
}

//endregion

//region AdjunctEntityIdNumInteger

pub type AdjunctEntityIdNumInteger = id_num::IntegerIdNum;

impl AdjunctEntityIdNumDefinition for AdjunctEntityIdNumInteger {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        self.collect_symbol_refs()
    }
}

//endregion

//region AdjunctEntityOrdering

#[derive(Clone, Debug)]
pub enum AdjunctEntityOrdering {
    Unordered,
    Ordered,
}

impl Default for AdjunctEntityOrdering {
    fn default() -> AdjunctEntityOrdering {
        AdjunctEntityOrdering::Unordered
    }
}

impl convert::TryFrom<String> for AdjunctEntityOrdering {
    type Error = String;

    fn try_from(s: String) -> result::Result<Self, Self::Error> {
        match s.as_ref() {
            "unordered" | "" => Ok(AdjunctEntityOrdering::Unordered),
            "ordered" => Ok(AdjunctEntityOrdering::Ordered),
            _ => Err(format!("Unrecognized AdjunctEntityOrdering value {}", s).to_owned()),
        }
    }
}

//endregion
