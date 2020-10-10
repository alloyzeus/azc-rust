//

use std::convert::{self, TryInto};

use crate::azml::{ref_key, yaml};

//region RefKeyYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RefKeyYaml {}

impl convert::TryFrom<&RefKeyYaml> for ref_key::RefKey {
    type Error = yaml::Error;
    fn try_from(_x: &RefKeyYaml) -> Result<Self, Self::Error> {
        Ok(ref_key::RefKey {})
    }
}

impl convert::TryFrom<RefKeyYaml> for ref_key::RefKey {
    type Error = yaml::Error;
    fn try_from(x: RefKeyYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//endregion
