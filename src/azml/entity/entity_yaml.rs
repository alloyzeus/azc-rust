//

use serde::{Deserialize, Serialize};
use std::{convert, convert::TryInto};

use crate::azml::{entity::entity, entity::entity_id_yaml, mixin, mixin_yaml, yaml};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EntityYaml {
    #[serde(default)]
    documentation: String,

    id: entity_id_yaml::EntityIdYaml,

    #[serde(default)]
    service: Option<EntityServiceYaml>,

    mixins: Vec<mixin_yaml::MixinYaml>,
}

impl convert::TryFrom<EntityYaml> for entity::Entity {
    type Error = yaml::Error;

    fn try_from(x: EntityYaml) -> Result<Self, Self::Error> {
        Ok(entity::Entity {
            documentation: x.documentation,
            id: x.id.try_into()?,
            service: if let Some(service) = x.service {
                Some(service.into())
            } else {
                None
            },
            mixins: x
                .mixins
                .into_iter()
                .map(|x| mixin::Mixin::try_from(x))
                .collect::<Result<Vec<mixin::Mixin>, _>>()?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
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
