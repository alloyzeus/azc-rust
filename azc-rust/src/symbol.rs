//

use crate::symbol_kind;

#[derive(Debug)]
pub struct Symbol {
    pub identifier: String,
    pub kind: symbol_kind::SymbolKind,
    //TODO: should be SymbolParameters
    pub parameters: Option<Box<dyn std::any::Any>>,
}

pub trait SymbolParameters: 'static + std::fmt::Debug {}
