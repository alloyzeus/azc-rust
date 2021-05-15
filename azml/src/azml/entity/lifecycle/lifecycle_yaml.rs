//

use std::convert::{self, TryInto};

use crate::azml::yaml;

use super::{
    creation::creation_yaml, deletion::deletion_yaml, expiration::expiration_yaml, lifecycle,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LifecycleYaml {
    creation: creation_yaml::CreationYaml,

    #[serde(default)]
    deletion: deletion_yaml::DeletionYaml,

    #[serde(default)]
    expiration: expiration_yaml::ExpirationYaml,
}

impl convert::TryFrom<&LifecycleYaml> for lifecycle::Lifecycle {
    type Error = yaml::Error;

    fn try_from(x: &LifecycleYaml) -> Result<Self, Self::Error> {
        Ok(lifecycle::Lifecycle {
            creation: (&x.creation).try_into()?,
            deletion: (&x.deletion).try_into()?,
            expiration: (&x.expiration).try_into()?,
        })
    }
}

impl convert::TryFrom<LifecycleYaml> for lifecycle::Lifecycle {
    type Error = yaml::Error;

    fn try_from(x: LifecycleYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}
