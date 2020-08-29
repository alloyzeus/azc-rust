//

use crate::symbol_kind;

#[derive(Debug)]
pub struct Symbol {
    pub identifier: String,
    pub kind: symbol_kind::SymbolKind,
    pub parameters: Option<Box<dyn SymbolParameters + 'static>>,
}

pub trait SymbolParameters: 'static + std::fmt::Debug {}
