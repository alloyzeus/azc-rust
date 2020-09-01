//

use crate::{base::arity, symbol};
use std::{convert, result};

#[derive(Debug)]
pub struct Adjunct {
    pub kind: AdjunctKind,

    pub hosts: Vec<AdjunctHost>,

    pub arity: arity::ArityConstraint,

    pub parameters: Option<Box<dyn AdjuctContent>>,
}

pub trait AdjuctContent: mopa::Any + std::fmt::Debug {}

mopafy!(AdjuctContent);

impl symbol::SymbolParameters for Adjunct {}

#[derive(Debug)]
pub struct AdjunctHost {
    pub name: String,
}

#[derive(Debug)]
pub enum AdjunctKind {
    ValueObject,
    // Represented by AdjuctEntityDefinition
    Entity,
}

#[derive(Debug)]
pub struct AdjunctEntityDefinition {
    pub ordering: AdjunctEntityOrdering,
}

impl AdjuctContent for AdjunctEntityDefinition {}

#[derive(Debug)]
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
    type Error = &'static str;

    fn try_from(s: String) -> result::Result<Self, Self::Error> {
        match s.as_ref() {
            "unordered" | "" => Ok(AdjunctEntityOrdering::Unordered),
            "ordered" => Ok(AdjunctEntityOrdering::Ordered),
            _ => Err("Unrecognized"),
        }
    }
}
