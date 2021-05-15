//

use std::convert::{self, TryInto};

use crate::azml::{error, yaml};

use super::expiration;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ExpirationYaml {
    #[serde(default)]
    enabled: bool,

    #[serde(default)]
    runtime_overrideable: bool,
}

impl Default for ExpirationYaml {
    fn default() -> Self {
        let x = expiration::Expiration::default();
        Self {
            enabled: x.enabled,
            runtime_overrideable: x.runtime_overrideable,
        }
    }
}

impl convert::TryFrom<&ExpirationYaml> for expiration::Expiration {
    type Error = yaml::Error;

    fn try_from(x: &ExpirationYaml) -> Result<Self, Self::Error> {
        Ok(expiration::Expiration {
            enabled: x.enabled,
            runtime_overrideable: x.runtime_overrideable,
        })
    }
}

impl convert::TryFrom<ExpirationYaml> for expiration::Expiration {
    type Error = error::Error;

    fn try_from(x: ExpirationYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}
