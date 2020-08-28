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
        println!("self {:?}", self.service);
        entity::Entity {
            description: self.description,
            service: if self.service.is_some() {
                Some(self.service.unwrap().into())
            } else {
                None
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct EntityServiceSerde {
    description: String,
}

impl Into<entity::EntityService> for EntityServiceSerde {
    fn into(self) -> entity::EntityService {
        entity::EntityService {
            description: self.description,
        }
    }
}