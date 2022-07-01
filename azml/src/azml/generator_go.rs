//

use super::generator;

#[derive(Clone, Debug)]
pub struct GeneratorGoOptions {
    pub package_identifier: String,
}

impl generator::GeneratorOptions for GeneratorGoOptions {}
