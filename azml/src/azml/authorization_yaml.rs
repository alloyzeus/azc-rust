//

use std::convert::{self, TryInto};

use super::{authorization, yaml};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AuthorizationYaml {
    #[serde(default)]
    same_process: AuthorizationSpecYaml,
    #[serde(default)]
    same_realm: AuthorizationSpecYaml,
    #[serde(default)]
    anywhere: AuthorizationSpecYaml,
}

impl Default for AuthorizationYaml {
    fn default() -> Self {
        Self {
            same_process: AuthorizationSpecYaml::default(),
            same_realm: AuthorizationSpecYaml::default(),
            anywhere: AuthorizationSpecYaml::default(),
        }
    }
}

impl convert::TryFrom<AuthorizationYaml> for authorization::Authorization {
    type Error = yaml::Error;

    fn try_from(x: AuthorizationYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

impl convert::TryFrom<&AuthorizationYaml> for authorization::Authorization {
    type Error = yaml::Error;

    fn try_from(x: &AuthorizationYaml) -> Result<Self, Self::Error> {
        Ok(Self {
            same_process: (&x.same_process).try_into()?,
            same_realm: (&x.same_realm).try_into()?,
            anywhere: (&x.anywhere).try_into()?,
        })
    }
}

// ----

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AuthorizationSpecYaml {
    #[serde(default)]
    allow: String,
}

impl Default for AuthorizationSpecYaml {
    fn default() -> Self {
        return Self {
            allow: authorization::AuthorizationAllow::default().into(),
        };
    }
}

impl convert::TryFrom<AuthorizationSpecYaml> for authorization::AuthorizationSpec {
    type Error = yaml::Error;

    fn try_from(x: AuthorizationSpecYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

impl convert::TryFrom<&AuthorizationSpecYaml> for authorization::AuthorizationSpec {
    type Error = yaml::Error;

    fn try_from(x: &AuthorizationSpecYaml) -> Result<Self, Self::Error> {
        Ok(Self {
            allow: (&x.allow).try_into()?,
        })
    }
}
