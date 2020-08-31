//

use crate::{entity, mixin_serde};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EntitySerde {
    #[serde(default)]
    documentation: String,

    #[serde(default)]
    service: Option<EntityServiceSerde>,

    mixins: Vec<mixin_serde::MixinSerde>,
}

impl From<EntitySerde> for entity::Entity {
    fn from(x: EntitySerde) -> entity::Entity {
        entity::Entity {
            documentation: x.documentation,
            service: if let Some(service) = x.service {
                Some(service.into())
            } else {
                None
            },
            mixins: x.mixins.into_iter().map(|x| x.into()).collect(),
        }
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
