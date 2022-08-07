//TODO: this should go to plugin

use std::convert;

use super::{generator_go, yaml};

//region GeneratorGoOptionsYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GeneratorGoOptionsYaml {
    package_identifier: String,

    #[serde(default)]
    azfl_package_uri: String,

    #[serde(default)]
    package_opts: Vec<GeneratorGoPackageOptionsYaml>,
}

impl convert::TryFrom<GeneratorGoOptionsYaml> for generator_go::GeneratorGoOptions {
    type Error = yaml::Error;

    fn try_from(x: GeneratorGoOptionsYaml) -> Result<Self, Self::Error> {
        Ok(Self {
            package_identifier: x.package_identifier.to_owned(),
            azfl_package_uri: x.azfl_package_uri.to_owned(),
            package_opts: x
                .package_opts
                .into_iter()
                .map(|o| generator_go::GeneratorGoPackageOptions::from(&o))
                .collect::<Vec<generator_go::GeneratorGoPackageOptions>>(),
        })
    }
}

//endregion

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GeneratorGoPackageOptionsYaml {
    identifier: String,
    uri: String,
}

impl convert::From<&GeneratorGoPackageOptionsYaml> for generator_go::GeneratorGoPackageOptions {
    fn from(x: &GeneratorGoPackageOptionsYaml) -> Self {
        Self {
            identifier: x.identifier.to_owned(),
            uri: x.uri.to_owned(),
        }
    }
}
