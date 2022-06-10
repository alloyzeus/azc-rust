//

use std::convert::{self, TryInto};

use crate::azml::{
    adjunct::adjunct_value,
    entity::{abstract_, abstract_yaml},
    yaml,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctPrimeYaml {
    #[serde(default)]
    documentation: String,

    #[serde(default)]
    implements: Vec<abstract_yaml::AbstractImplementationYaml>,

    #[serde(default)]
    identity: AdjunctPrimeIdentityYaml,
}

impl convert::TryFrom<&AdjunctPrimeYaml> for adjunct_value::AdjunctPrime {
    type Error = yaml::Error;

    fn try_from(x: &AdjunctPrimeYaml) -> Result<Self, Self::Error> {
        Ok(adjunct_value::AdjunctPrime {
            documentation: x.documentation.to_owned(),
            implements: x
                .implements
                .iter()
                .map(|x| abstract_::AbstractImplementation::try_from(x))
                .collect::<Result<Vec<abstract_::AbstractImplementation>, _>>()?,
            identity: (&x.identity).try_into()?,
        })
    }
}

impl convert::TryFrom<AdjunctPrimeYaml> for adjunct_value::AdjunctPrime {
    type Error = yaml::Error;

    fn try_from(x: AdjunctPrimeYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctPrimeIdentityYaml {
    #[serde(default)]
    enabled: bool,

    #[serde(default)]
    prefix: String,
}

impl Default for AdjunctPrimeIdentityYaml {
    fn default() -> Self {
        Self {
            enabled: false,
            prefix: "".to_owned(),
        }
    }
}

impl convert::TryFrom<AdjunctPrimeIdentityYaml> for adjunct_value::AdjunctPrimeIdentity {
    type Error = yaml::Error;

    fn try_from(x: AdjunctPrimeIdentityYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

impl convert::TryFrom<&AdjunctPrimeIdentityYaml> for adjunct_value::AdjunctPrimeIdentity {
    type Error = yaml::Error;

    fn try_from(x: &AdjunctPrimeIdentityYaml) -> Result<Self, Self::Error> {
        Ok(adjunct_value::AdjunctPrimeIdentity {
            enabled: x.enabled,
            prefix: x.prefix.to_owned(),
        })
    }
}