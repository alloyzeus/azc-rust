//

use std::convert::{self, TryInto};

use crate::azml::yaml;

use super::creation;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreationYaml {
    #[serde(default)]
    documentation: String,

    #[serde(default)]
    allow_cross_process_callers: bool,
}

impl Default for CreationYaml {
    fn default() -> Self {
        Self {
            documentation: "".to_owned(),
            allow_cross_process_callers: false,
        }
    }
}

impl convert::TryFrom<&CreationYaml> for creation::Creation {
    type Error = yaml::Error;

    fn try_from(x: &CreationYaml) -> Result<Self, Self::Error> {
        Ok(creation::Creation {
            documentation: x.documentation.to_owned(),
            allow_cross_process_callers: x.allow_cross_process_callers,
        })
    }
}

impl convert::TryFrom<CreationYaml> for creation::Creation {
    type Error = yaml::Error;

    fn try_from(x: CreationYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}
