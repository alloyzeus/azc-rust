//

use super::symbol;

#[derive(Clone, Debug)]
pub struct Abstract {
    pub documentation: String,
}

impl symbol::SymbolDefinition for Abstract {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        //TODO: collect from definition
        Vec::new()
    }
}

#[derive(Clone, Debug)]
pub struct AbstractImplementation {
    pub kind: String,
    pub attributes: Vec<AbstractImplementationAttribute>,
}

#[derive(Clone, Debug)]
pub struct AbstractImplementationAttribute {
    pub identifier: String,
    pub kind: String,
}
