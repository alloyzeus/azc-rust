//

use crate::azml::symbol;

#[derive(Debug)]
pub struct ModuleDefinition {
    pub symbols: Vec<symbol::Symbol>,
}
