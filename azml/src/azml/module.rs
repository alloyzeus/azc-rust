//

use crate::azml::symbol;

#[derive(Clone, Debug)]
pub struct ModuleDefinition {
    //pub realms: Vec<String>,
    pub symbols: Vec<symbol::Symbol>,
}
