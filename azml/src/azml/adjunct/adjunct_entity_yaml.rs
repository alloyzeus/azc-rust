//

use std::convert::TryInto;

use crate::azml::adjunct::adjunct_entity;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctEntityYaml {
    #[serde(default)]
    ordering: String,
}

impl From<AdjunctEntityYaml> for adjunct_entity::AdjunctEntity {
    fn from(x: AdjunctEntityYaml) -> adjunct_entity::AdjunctEntity {
        adjunct_entity::AdjunctEntity {
            ordering: x.ordering.try_into().unwrap_or_default(),
        }
    }
}
