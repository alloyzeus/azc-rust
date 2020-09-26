//

#[derive(Debug)]
pub struct Symbol {
    pub identifier: String,
    pub parameters: Box<dyn SymbolDefinition>,
}

pub trait SymbolDefinition: mopa::Any + std::fmt::Debug {}

mopafy!(SymbolDefinition);
