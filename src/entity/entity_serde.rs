//

use serde::{Deserialize, Serialize};
use std::{convert, convert::TryInto};

use crate::{base::azml, entity::entity, entity::entity_id_serde, mixin, mixin_serde};

#[derive(Serialize, Deserialize)]
pub struct EntitySerde {
    #[serde(default)]
    documentation: String,

    id: entity_id_serde::EntityIdDefinitionSerde,

    #[serde(default)]
    service: Option<EntityServiceSerde>,

    mixins: Vec<mixin_serde::MixinSerde>,
}

impl convert::TryFrom<EntitySerde> for entity::Entity {
    type Error = azml::Error;

    fn try_from(x: EntitySerde) -> Result<Self, Self::Error> {
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
struct EntityServiceSerde {
    #[serde(default)]
    documentation: String,

    #[serde(default)]
    enabled: bool,
}

impl From<EntityServiceSerde> for entity::EntityService {
    fn from(x: EntityServiceSerde) -> entity::EntityService {
        entity::EntityService {
            documentation: x.documentation,
            enabled: x.enabled,
        }
    }
}
