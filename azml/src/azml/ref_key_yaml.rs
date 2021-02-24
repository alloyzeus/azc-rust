//

use std::convert::{self, TryInto};

use crate::azml::{ref_key, yaml};

//region RefKey

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct RefKeyYaml {
    #[serde(default)]
    pub azrs: RefKeyAzisYaml,
}

// impl convert::TryFrom<&RefKeyYaml> for ref_key::RefKey {
//     type Error = yaml::Error;
//     fn try_from(x: &RefKeyYaml) -> Result<Self, Self::Error> {
//         Ok(ref_key::RefKey {
//             azrs: x.azrs.try_into()?,
//         })
//     }
// }

impl convert::TryFrom<RefKeyYaml> for ref_key::RefKey {
    type Error = yaml::Error;
    fn try_from(x: RefKeyYaml) -> Result<Self, Self::Error> {
        Ok(ref_key::RefKey {
            azrs: x.azrs.try_into()?,
        })
    }
}

//endregion

//region RefKeyAzis

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct RefKeyAzisYaml {
    #[serde(default)]
    pub prefix: String,
}

impl convert::TryFrom<&RefKeyAzisYaml> for ref_key::RefKeyAzis {
    type Error = yaml::Error;
    fn try_from(x: &RefKeyAzisYaml) -> Result<Self, Self::Error> {
        Ok(ref_key::RefKeyAzis {
            prefix: x.prefix.to_owned(),
        })
    }
}

impl convert::TryFrom<RefKeyAzisYaml> for ref_key::RefKeyAzis {
    type Error = yaml::Error;
    fn try_from(x: RefKeyAzisYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//endregion
