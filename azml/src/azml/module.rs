//

use crate::azml::symbol;

#[derive(Clone, Debug)]
pub struct ModuleDefinition {
    pub symbols: Vec<symbol::Symbol>,
}
