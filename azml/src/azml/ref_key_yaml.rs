//

use std::convert::{self, TryInto};

use crate::azml::{ref_key, yaml};

//region RefKey

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct RefKeyYaml {
    #[serde(default)]
    pub azer_text: RefKeyAzerTextYaml,
}

// impl convert::TryFrom<&RefKeyYaml> for ref_key::RefKey {
//     type Error = yaml::Error;
//     fn try_from(x: &RefKeyYaml) -> Result<Self, Self::Error> {
//         Ok(ref_key::RefKey {
//             azer_text: x.azer_text.try_into()?,
//         })
//     }
// }

impl convert::TryFrom<RefKeyYaml> for ref_key::RefKey {
    type Error = yaml::Error;
    fn try_from(x: RefKeyYaml) -> Result<Self, Self::Error> {
        Ok(ref_key::RefKey {
            azer_text: x.azer_text.try_into()?,
        })
    }
}

//endregion

//region RefKeyAzerText

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct RefKeyAzerTextYaml {
    #[serde(default)]
    pub prefix: String,
}

impl convert::TryFrom<&RefKeyAzerTextYaml> for ref_key::RefKeyAzerText {
    type Error = yaml::Error;
    fn try_from(x: &RefKeyAzerTextYaml) -> Result<Self, Self::Error> {
        Ok(ref_key::RefKeyAzerText {
            prefix: x.prefix.to_owned(),
        })
    }
}

impl convert::TryFrom<RefKeyAzerTextYaml> for ref_key::RefKeyAzerText {
    type Error = yaml::Error;
    fn try_from(x: RefKeyAzerTextYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//endregion
