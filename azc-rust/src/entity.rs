//

use serde::{Deserialize, Serialize};

use crate::module;

#[derive(Debug)]
pub struct Entity {
    description: String,
    service: Option<EntityService>,
}

impl module::SymbolParameters for Entity {}

#[derive(Debug)]
struct EntityService {
    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct EntitySerde {
    #[serde(default)]
    description: String,

    #[serde(default)]
    service: Option<EntityServiceSerde>,
}

impl Into<Entity> for EntitySerde {
    fn into(self) -> Entity {
        println!("self {:?}", self.service);
        Entity {
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

impl Into<EntityService> for EntityServiceSerde {
    fn into(self) -> EntityService {
        EntityService {
            description: self.description,
        }
    }
}
