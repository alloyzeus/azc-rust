//

use super::entity;

//region EntityServiceYaml

#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
pub struct EntityServiceYaml {
    #[serde(default)]
    documentation: String,

    #[serde(default)]
    enabled: bool,
}

impl From<&EntityServiceYaml> for entity::EntityService {
    fn from(x: &EntityServiceYaml) -> entity::EntityService {
        entity::EntityService {
            documentation: x.documentation.to_owned(),
            enabled: x.enabled,
        }
    }
}

impl From<EntityServiceYaml> for entity::EntityService {
    fn from(x: EntityServiceYaml) -> entity::EntityService {
        (&x).into()
    }
}

//endregion
