//

use crate::azml::{adjunct::adjunct, symbol};

#[derive(Clone, Debug)]
pub struct AdjunctValueObject {/* attributes or reference to the value object */}

impl adjunct::AdjuctDefinition for AdjunctValueObject {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        Vec::new()
    }
}
