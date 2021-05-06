//

use crate::azml::{adjunct::adjunct, symbol};

#[derive(Clone, Debug)]
pub struct AdjunctValueObject {
    pub kind: String,
}

impl adjunct::AdjuctDefinition for AdjunctValueObject {}

impl symbol::SymbolDefinition for AdjunctValueObject {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        Vec::new()
    }
}
