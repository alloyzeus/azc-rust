//

use super::generator;

#[derive(Clone, Debug)]
pub struct GeneratorGoOptions {
    pub packages: GeneratorGoPackagesOptions,
    pub azfl_package_uri: String,

    pub imports: Vec<GeneratorGoImportPackageOptions>,
}

impl generator::GeneratorOptions for GeneratorGoOptions {}

#[derive(Clone, Debug)]
pub struct GeneratorGoImportPackageOptions {
    pub identifier: String,
    pub uri: String,
}

#[derive(Clone, Debug)]
pub struct GeneratorGoPackagesOptions {
    pub contract: String,
    pub server: String,
    pub client: String,
}

impl Default for GeneratorGoPackagesOptions {
    fn default() -> Self {
        GeneratorGoPackagesOptions{
            contract: "".to_owned(),
            server: "".to_owned(),
            client: "".to_owned(),
        }
    }
}
