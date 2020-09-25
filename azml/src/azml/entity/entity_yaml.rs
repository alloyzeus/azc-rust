//

use std::convert::{self, TryInto};

use crate::azml::{
    entity::{entity, entity_id_yaml},
    mixin, mixin_yaml, yaml,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EntityYaml {
    #[serde(default)]
    documentation: String,

    id: entity_id_yaml::EntityIdYaml,

    creation: EntityCreationYaml,
    mixins: Vec<mixin_yaml::MixinYaml>,

    #[serde(default)]
    service: Option<EntityServiceYaml>,
}

impl convert::TryFrom<EntityYaml> for entity::Entity {
    type Error = yaml::Error;

    fn try_from(x: EntityYaml) -> Result<Self, Self::Error> {
        Ok(entity::Entity {
            documentation: x.documentation,
            id: x.id.try_into()?,
            creation: x.creation.try_into()?,
            mixins: x
                .mixins
                .into_iter()
                .map(|x| mixin::Mixin::try_from(x))
                .collect::<Result<Vec<mixin::Mixin>, _>>()?,
            service: if let Some(service) = x.service {
                Some(service.into())
            } else {
                None
            },
        })
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct EntityCreationYaml {
    #[serde(default)]
    documentation: String,
}

impl convert::TryFrom<EntityCreationYaml> for entity::EntityCreation {
    type Error = yaml::Error;

    fn try_from(x: EntityCreationYaml) -> Result<Self, Self::Error> {
        Ok(entity::EntityCreation {
            documentation: x.documentation,
        })
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct EntityServiceYaml {
    #[serde(default)]
    documentation: String,

    #[serde(default)]
    enabled: bool,
}

impl From<EntityServiceYaml> for entity::EntityService {
    fn from(x: EntityServiceYaml) -> entity::EntityService {
        entity::EntityService {
            documentation: x.documentation,
            enabled: x.enabled,
        }
    }
}
