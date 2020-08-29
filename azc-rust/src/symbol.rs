//

use crate::symbol_kind;

#[derive(Debug)]
pub struct Symbol {
    pub identifier: String,
    pub kind: symbol_kind::SymbolKind,
    pub parameters: Option<Box<dyn SymbolParameters>>,
}

pub trait SymbolParameters: mopa::Any + std::fmt::Debug {}

mopafy!(SymbolParameters);
