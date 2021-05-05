//

use std::convert::{self, TryInto};

use crate::azml::{adjunct::adjunct_cardinal, yaml};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctCardinalYaml {
    #[serde(default)]
    documentation: String,
}

impl convert::TryFrom<&AdjunctCardinalYaml> for adjunct_cardinal::AdjunctCardinal {
    type Error = yaml::Error;

    fn try_from(x: &AdjunctCardinalYaml) -> Result<Self, Self::Error> {
        Ok(adjunct_cardinal::AdjunctCardinal {
            documentation: x.documentation.to_owned(),
        })
    }
}

impl convert::TryFrom<AdjunctCardinalYaml> for adjunct_cardinal::AdjunctCardinal {
    type Error = yaml::Error;

    fn try_from(x: AdjunctCardinalYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}
