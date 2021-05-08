//

use std::convert::{self, TryInto};

use crate::azml::{adjunct::adjunct_prime, yaml};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctPrimeYaml {
    #[serde(default)]
    documentation: String,
}

impl convert::TryFrom<&AdjunctPrimeYaml> for adjunct_prime::AdjunctPrime {
    type Error = yaml::Error;

    fn try_from(x: &AdjunctPrimeYaml) -> Result<Self, Self::Error> {
        Ok(adjunct_prime::AdjunctPrime {
            documentation: x.documentation.to_owned(),
        })
    }
}

impl convert::TryFrom<AdjunctPrimeYaml> for adjunct_prime::AdjunctPrime {
    type Error = yaml::Error;

    fn try_from(x: AdjunctPrimeYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}
