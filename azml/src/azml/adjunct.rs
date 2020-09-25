//

use crate::azml::{arity, symbol};
use std::{convert, result};

#[derive(Debug)]
pub struct Adjunct {
    pub hosts: Vec<AdjunctHost>,

    pub arity: arity::ArityConstraint,

    pub parameters: Box<dyn AdjuctDefinition>,
}

pub trait AdjuctDefinition: mopa::Any + std::fmt::Debug {}

mopafy!(AdjuctDefinition);

impl symbol::SymbolDefinition for Adjunct {}

#[derive(Debug)]
pub struct AdjunctHost {
    pub name: String,
}

#[derive(Debug)]
pub struct AdjunctEntity {
    pub ordering: AdjunctEntityOrdering,
}

impl AdjuctDefinition for AdjunctEntity {}

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

#[derive(Debug)]
pub struct AdjunctValueObject {}

impl AdjuctDefinition for AdjunctValueObject {}
