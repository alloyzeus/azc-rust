//

use std::convert::{self, TryInto};

use crate::azml::yaml;

use super::ref_key;

//region RefKey

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct RefKeyYaml {
    #[serde(default)]
    pub azid_text: RefKeyAzidTextYaml,
}

impl convert::TryFrom<&RefKeyYaml> for ref_key::RefKey {
    type Error = yaml::Error;
    fn try_from(x: &RefKeyYaml) -> Result<Self, Self::Error> {
        Ok(ref_key::RefKey {
            azid_text: (&x.azid_text).try_into()?,
        })
    }
}

impl convert::TryFrom<RefKeyYaml> for ref_key::RefKey {
    type Error = yaml::Error;
    fn try_from(x: RefKeyYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//endregion

//region RefKeyAzidText

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct RefKeyAzidTextYaml {
    #[serde(default)]
    pub prefix: String,
}

impl convert::TryFrom<&RefKeyAzidTextYaml> for ref_key::RefKeyAzidText {
    type Error = yaml::Error;
    fn try_from(x: &RefKeyAzidTextYaml) -> Result<Self, Self::Error> {
        Ok(ref_key::RefKeyAzidText {
            prefix: x.prefix.to_owned(),
        })
    }
}

impl convert::TryFrom<RefKeyAzidTextYaml> for ref_key::RefKeyAzidText {
    type Error = yaml::Error;
    fn try_from(x: RefKeyAzidTextYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//endregion
