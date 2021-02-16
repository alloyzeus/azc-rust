//

use std::{
    convert::{self, TryInto},
    result,
};

use crate::azml::{abstract_, adjunct::adjunct, attribute, eid};

//region AdjunctEntity

#[derive(Clone, Debug)]
pub struct AdjunctEntity {
    pub ordering: AdjunctEntityOrdering,
    pub id: AdjunctEntityId,
    pub implements: abstract_::AbstractImplementation,
    pub scope: AdjunctEntityScope,
    pub attributes: Vec<attribute::Attribute>,
}

impl adjunct::AdjuctDefinition for AdjunctEntity {}

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

//region AdjunctEntityId

#[derive(Clone, Debug)]
pub struct AdjunctEntityId {
    pub definition: Box<dyn AdjunctEntityIdDefinition>,
}

//endregion

//region AdjunctEntityIdDefinition

pub trait AdjunctEntityIdDefinition:
    mopa::Any + AdjunctEntityIdDefinitionClone + std::fmt::Debug
{
}

mopafy!(AdjunctEntityIdDefinition);

pub trait AdjunctEntityIdDefinitionClone {
    fn clone_box(&self) -> Box<dyn AdjunctEntityIdDefinition>;
}

impl<T> AdjunctEntityIdDefinitionClone for T
where
    T: AdjunctEntityIdDefinition + Clone,
{
    fn clone_box(&self) -> Box<dyn AdjunctEntityIdDefinition> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn AdjunctEntityIdDefinition> {
    fn clone(&self) -> Box<dyn AdjunctEntityIdDefinition> {
        self.clone_box()
    }
}

//endregion

//region AdjunctEntityIdInteger

pub type AdjunctEntityIdInteger = eid::IntegerId;

impl AdjunctEntityIdDefinition for AdjunctEntityIdInteger {}

pub trait AdjunctEntityIdIntegerEncoding: mopa::Any + std::fmt::Debug {}

mopafy!(AdjunctEntityIdIntegerEncoding);

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
