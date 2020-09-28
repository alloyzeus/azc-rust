//

use std::{convert, result};

use crate::azml::adjunct::adjunct;

#[derive(Clone, Debug)]
pub struct AdjunctEntity {
    pub ordering: AdjunctEntityOrdering,
    pub id: AdjunctEntityId,
}

impl adjunct::AdjuctDefinition for AdjunctEntity {}

#[derive(Clone, Debug)]
pub struct AdjunctEntityId {
    // A flag to indicate that an ID is globally-unique; a more accurate term
    // would be system-wide-unique. A globally-unique
    // means that an instance of adjunct can be addressed directly while
    // being an adjunct.
    //
    // Some example of adjuncts with globally-unique IDs are shop items
    // in a marketplace. They usually get globally-unique IDs to hide the
    // store where it actually belongs to.
    //
    // Enabling this flag requires the ordering to be Unordered.
    pub unique: bool,
}

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
