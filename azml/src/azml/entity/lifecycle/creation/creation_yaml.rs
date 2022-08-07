//

use std::convert::{self, TryInto};

use crate::azml::{authorization_yaml, yaml};

use super::creation;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreationYaml {
    #[serde(default)]
    documentation: String,

    #[serde(default)]
    authorization: authorization_yaml::AuthorizationYaml,
}

impl Default for CreationYaml {
    fn default() -> Self {
        Self {
            documentation: "".to_owned(),
            authorization: authorization_yaml::AuthorizationYaml::default(),
        }
    }
}

impl convert::TryFrom<&CreationYaml> for creation::Creation {
    type Error = yaml::Error;

    fn try_from(x: &CreationYaml) -> Result<Self, Self::Error> {
        Ok(Self {
            documentation: x.documentation.to_owned(),
            authorization: (&x.authorization).try_into()?,
        })
    }
}

impl convert::TryFrom<CreationYaml> for creation::Creation {
    type Error = yaml::Error;

    fn try_from(x: CreationYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}
