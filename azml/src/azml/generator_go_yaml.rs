//TODO: this should go to plugin

use std::convert;

use super::{generator_go, yaml};

//region GeneratorGoOptionsYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GeneratorGoOptionsYaml {
    package_identifier: String,
}

impl convert::TryFrom<GeneratorGoOptionsYaml> for generator_go::GeneratorGoOptions {
    type Error = yaml::Error;

    fn try_from(x: GeneratorGoOptionsYaml) -> Result<Self, Self::Error> {
        Ok(generator_go::GeneratorGoOptions {
            package_identifier: x.package_identifier.clone(),
        })
    }
}

//endregion
