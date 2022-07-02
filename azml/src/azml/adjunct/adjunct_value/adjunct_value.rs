//

use crate::azml::{adjunct::adjunct, entity::abstract_, symbol};

// NOTE: check the note in adjunct_prime

#[derive(Clone, Debug)]
pub struct AdjunctValue {
    pub documentation: String,
    pub implements: Vec<abstract_::AbstractImplementation>,
    pub kind: String,
}

impl adjunct::AdjuctDefinition for AdjunctValue {}

impl symbol::SymbolDefinition for AdjunctValue {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        vec![symbol::SymbolRef::from(self.kind.to_owned())]
    }
}
