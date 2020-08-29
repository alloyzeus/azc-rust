//

use crate::entity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EntitySerde {
    #[serde(default)]
    description: String,

    #[serde(default)]
    service: Option<EntityServiceSerde>,
}

impl Into<entity::Entity> for EntitySerde {
    fn into(self) -> entity::Entity {
        entity::Entity {
            description: self.description,
            service: if let Some(service) = self.service {
                Some(service.into())
            } else {
                None
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct EntityServiceSerde {
    //TODO: remove this
    #[serde(default)]
    description: String,
}

impl Into<entity::EntityService> for EntityServiceSerde {
    fn into(self) -> entity::EntityService {
        entity::EntityService {
            description: self.description,
        }
    }
}
