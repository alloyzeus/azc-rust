//TODO: this should go to plugin

use std::convert;

use super::{generator_go, yaml};

//region GeneratorGoOptionsYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GeneratorGoOptionsYaml {
    #[serde(default)]
    packages: GeneratorGoPackagesOptionsYaml,

    #[serde(default)]
    azfl_package_uri: String,

    #[serde(default)]
    imports: Vec<GeneratorGoImportPackageOptionsYaml>,
}

impl convert::TryFrom<GeneratorGoOptionsYaml> for generator_go::GeneratorGoOptions {
    type Error = yaml::Error;

    fn try_from(x: GeneratorGoOptionsYaml) -> Result<Self, Self::Error> {
        Ok(Self {
            packages: (&x.packages).into(),
            azfl_package_uri: x.azfl_package_uri.to_owned(),
            imports: x
                .imports
                .into_iter()
                .map(|o| generator_go::GeneratorGoImportPackageOptions::from(&o))
                .collect::<Vec<generator_go::GeneratorGoImportPackageOptions>>(),
        })
    }
}

//endregion

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GeneratorGoImportPackageOptionsYaml {
    identifier: String,
    uri: String,
}

impl convert::From<&GeneratorGoImportPackageOptionsYaml> for generator_go::GeneratorGoImportPackageOptions {
    fn from(x: &GeneratorGoImportPackageOptionsYaml) -> Self {
        Self {
            identifier: x.identifier.to_owned(),
            uri: x.uri.to_owned(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct GeneratorGoPackagesOptionsYaml {
    #[serde(default)]
    contract: String,

    #[serde(default)]
    server: String,

    #[serde(default)]
    client: String,
}

impl convert::From<&GeneratorGoPackagesOptionsYaml> for generator_go::GeneratorGoPackagesOptions {
    fn from(x: &GeneratorGoPackagesOptionsYaml) -> Self {
        Self {
            contract: x.contract.to_owned(),
            server: x.server.to_owned(),
            client: x.client.to_owned(),
        }
    }
}
