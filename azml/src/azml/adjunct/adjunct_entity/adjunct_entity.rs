//

use std::{
    convert::{self, TryInto},
    result,
};

use crate::azml::{
    attribute,
    entity::{
        abstract_, entity,
        id::{id_num, ref_key},
        lifecycle,
    },
    symbol,
};

use super::super::adjunct;

//region AdjunctEntity

//TODO: cardinality

#[derive(Clone, Debug)]
pub struct AdjunctEntity {
    pub id: AdjunctEntityId,
    pub identity: AdjunctEntityIdentity,
    //TODO: put into AdjunctEntityId?
    pub ordering: AdjunctEntityOrdering,
    pub implements: Vec<abstract_::AbstractImplementation>,
    pub lifecycle: lifecycle::Lifecycle,
    pub attributes: Vec<attribute::Attribute>,
}

impl entity::Entity for AdjunctEntity {}

impl adjunct::AdjuctDefinition for AdjunctEntity {}

impl symbol::SymbolDefinition for AdjunctEntity {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        let a_syms = self
            .attributes
            .iter()
            .fold(Vec::<symbol::SymbolRef>::new(), |a, b| {
                a.into_iter()
                    .chain(b.collect_symbol_refs())
                    .collect::<Vec<_>>()
            });
        let id_syms = self.id.num.definition.collect_symbol_refs();
        a_syms
            .iter()
            .chain(id_syms.iter())
            .map(|x| x.clone())
            .collect()
    }
}

//endregion

#[derive(Clone, PartialEq, Debug)]
pub enum AdjunctEntityIdentity {
    // Adjunct entity is using ref-key as its identity, i.e., it requires
    // complete hosts' identifiers to uniquely identify an instance.
    RefKey,
    // Adjunct entity  is using id-num as its identity, i.e., id-num is
    // unique across instances. An instance could be identified with only
    // an id-num.
    //
    // An example of adjuncts with id-num identity are shop items
    // in a marketplace. Some marketplace systems provide URLs which refer
    // to the items directly without giving any information about the store
    // these items belong to. It shows, e.g., https://example.com/items/12345678
    // instead of https://example.com/stores/345/items/12345678
    IdNum,
}

impl Default for AdjunctEntityIdentity {
    fn default() -> Self {
        Self::RefKey
    }
}

impl convert::TryFrom<String> for AdjunctEntityIdentity {
    type Error = String;

    fn try_from(s: String) -> result::Result<Self, Self::Error> {
        let sr: &str = s.as_ref();
        sr.try_into()
    }
}

impl convert::TryFrom<&String> for AdjunctEntityIdentity {
    type Error = String;

    fn try_from(s: &String) -> result::Result<Self, Self::Error> {
        let sr: &str = s.as_ref();
        sr.try_into()
    }
}

impl convert::TryFrom<&str> for AdjunctEntityIdentity {
    type Error = String;

    fn try_from(s: &str) -> result::Result<Self, Self::Error> {
        match s {
            "" | "ref_key" => Ok(Self::RefKey),
            "id_num" => Ok(Self::IdNum),
            _ => Err(format!("Unrecognized AdjunctEntityIdentity value {}", s).to_owned()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AdjunctEntityId {
    pub num: AdjunctEntityIdNum,
    pub ref_key: ref_key::RefKey,
}

//region AdjunctEntityIdNum

#[derive(Clone, Debug)]
pub struct AdjunctEntityIdNum {
    pub definition: Box<dyn AdjunctEntityIdNumDefinition>,
}

//endregion

//region AdjunctEntityIdNumDefinition

pub trait AdjunctEntityIdNumDefinition:
    mopa::Any + AdjunctEntityIdNumDefinitionClone + std::fmt::Debug + id_num::IdNumDefinition
{
}

mopafy!(AdjunctEntityIdNumDefinition);

pub trait AdjunctEntityIdNumDefinitionClone {
    fn clone_boxed_adjunct_entity_id_num_definition(&self)
        -> Box<dyn AdjunctEntityIdNumDefinition>;
}

impl<T> AdjunctEntityIdNumDefinitionClone for T
where
    T: AdjunctEntityIdNumDefinition + Clone,
{
    fn clone_boxed_adjunct_entity_id_num_definition(
        &self,
    ) -> Box<dyn AdjunctEntityIdNumDefinition> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn AdjunctEntityIdNumDefinition> {
    fn clone(&self) -> Box<dyn AdjunctEntityIdNumDefinition> {
        self.clone_boxed_adjunct_entity_id_num_definition()
    }
}

//endregion

//region AdjunctEntityIdNumInteger

pub type AdjunctEntityIdNumInteger = id_num::IntegerIdNum;

impl AdjunctEntityIdNumDefinition for AdjunctEntityIdNumInteger {}

//endregion

//region AdjunctEntityOrdering

#[derive(Clone, Debug)]
pub enum AdjunctEntityOrdering {
    Unordered,
    Ordered,
}

impl Default for AdjunctEntityOrdering {
    fn default() -> Self {
        Self::Unordered
    }
}

impl convert::TryFrom<String> for AdjunctEntityOrdering {
    type Error = String;

    fn try_from(s: String) -> result::Result<Self, Self::Error> {
        match s.as_ref() {
            "unordered" | "" => Ok(Self::Unordered),
            "ordered" => Ok(Self::Ordered),
            _ => Err(format!("Unrecognized AdjunctEntityOrdering value {}", s).to_owned()),
        }
    }
}

//endregion
