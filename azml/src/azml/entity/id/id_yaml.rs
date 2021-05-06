//

use std::convert::{self, TryInto};

use crate::azml::yaml;

use super::{id, id_num_yaml, ref_key_yaml};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct IdYaml {
    pub num: id_num_yaml::IdNumYaml,

    #[serde(default)]
    pub ref_key: ref_key_yaml::RefKeyYaml,
}

impl convert::TryFrom<&IdYaml> for id::Id {
    type Error = yaml::Error;

    fn try_from(x: &IdYaml) -> Result<Self, Self::Error> {
        Ok(id::Id {
            num: (&x.num).try_into()?,
            ref_key: (&x.ref_key).try_into()?,
        })
    }
}

impl convert::TryFrom<IdYaml> for id::Id {
    type Error = yaml::Error;

    fn try_from(x: IdYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}
