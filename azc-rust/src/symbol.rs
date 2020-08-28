//

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub kind: String,
    pub parameters: Option<Box<dyn SymbolParameters + 'static>>,
}

pub trait SymbolParameters: 'static + std::fmt::Debug {}
