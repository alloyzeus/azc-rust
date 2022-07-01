//

use super::generator;

#[derive(Clone, Debug)]
pub struct GeneratorGoOptions {
    pub package_identifier: String,
    pub azfl_package_uri: String,

    pub package_opts: Vec<GeneratorGoPackageOptions>,
}

impl generator::GeneratorOptions for GeneratorGoOptions {}

#[derive(Clone, Debug)]
pub struct GeneratorGoPackageOptions {
    pub identifier: String,
    pub uri: String,
}
