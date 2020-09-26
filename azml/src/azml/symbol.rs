//

use std::fmt;

#[derive(Clone, Debug)]
pub struct Symbol {
    pub identifier: String,
    pub parameters: Box<dyn SymbolDefinition>,
}

pub trait SymbolDefinition: mopa::Any + SymbolDefinitionClone + fmt::Debug {}

mopafy!(SymbolDefinition);

pub trait SymbolDefinitionClone {
    fn clone_box(&self) -> Box<dyn SymbolDefinition>;
}

impl<T> SymbolDefinitionClone for T
where
    T: SymbolDefinition + Clone,
{
    fn clone_box(&self) -> Box<dyn SymbolDefinition> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SymbolDefinition> {
    fn clone(&self) -> Box<dyn SymbolDefinition> {
        self.clone_box()
    }
}
