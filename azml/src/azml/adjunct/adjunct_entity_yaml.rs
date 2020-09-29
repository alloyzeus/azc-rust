//

use std::convert::{self, TryInto};

use crate::azml::{adjunct::adjunct_entity, yaml};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctEntityYaml {
    #[serde(default)]
    ordering: String,

    id: AdjunctEntityIdYaml,

    #[serde(default)]
    scope: String,
}

impl From<AdjunctEntityYaml> for adjunct_entity::AdjunctEntity {
    fn from(x: AdjunctEntityYaml) -> adjunct_entity::AdjunctEntity {
        adjunct_entity::AdjunctEntity {
            ordering: x.ordering.try_into().unwrap_or_default(),
            id: x.id.try_into().unwrap(),
            scope: x.scope.try_into().unwrap_or_default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctEntityIdYaml {
    unique: bool,
}

impl convert::TryFrom<AdjunctEntityIdYaml> for adjunct_entity::AdjunctEntityId {
    type Error = yaml::Error;

    fn try_from(x: AdjunctEntityIdYaml) -> Result<Self, Self::Error> {
        Ok(adjunct_entity::AdjunctEntityId { unique: x.unique })
    }
}
