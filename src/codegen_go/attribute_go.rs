//

use crate::codegen_go::symbol_go;

#[derive(Clone, Gtmpl)]
pub struct AttributeContext {
    pub identifier: String,
    pub type_name: String,
    pub kind: symbol_go::SymbolRefContext,
}
