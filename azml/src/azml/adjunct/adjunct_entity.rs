//

use std::{convert, result};

use crate::azml::adjunct::adjunct;

#[derive(Clone, Debug)]
pub struct AdjunctEntity {
    pub ordering: AdjunctEntityOrdering,
}

impl adjunct::AdjuctDefinition for AdjunctEntity {}

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
    type Error = &'static str;

    fn try_from(s: String) -> result::Result<Self, Self::Error> {
        match s.as_ref() {
            "unordered" | "" => Ok(AdjunctEntityOrdering::Unordered),
            "ordered" => Ok(AdjunctEntityOrdering::Ordered),
            _ => Err("Unrecognized"),
        }
    }
}
