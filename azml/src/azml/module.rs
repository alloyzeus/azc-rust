//

use std::collections::HashMap;

use crate::azml::{generator, symbol};

#[derive(Clone, Debug)]
pub struct ModuleDefinition {
    //pub realms: Vec<String>,
    pub symbols: Vec<symbol::Symbol>,

    pub options: HashMap<String, Box<dyn generator::GeneratorOptions>>,
}
