//

use std::{collections::HashMap, fmt};

use crate::azml::symbol;

#[derive(Clone, Debug)]
pub struct ModuleDefinition {
    //pub realms: Vec<String>,
    pub symbols: Vec<symbol::Symbol>,

    pub options: HashMap<String, String>,
}

//region GeneratorOption

pub trait GeneratorOption: mopa::Any + GeneratorOptionClone + fmt::Debug {}

mopafy!(GeneratorOption);

pub trait GeneratorOptionClone {
    fn clone_boxed_symbol_definition(&self) -> Box<dyn GeneratorOption>;
}

impl<T> GeneratorOptionClone for T
where
    T: GeneratorOption + Clone,
{
    fn clone_boxed_symbol_definition(&self) -> Box<dyn GeneratorOption> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn GeneratorOption> {
    fn clone(&self) -> Box<dyn GeneratorOption> {
        self.clone_boxed_symbol_definition()
    }
}

//endregion
