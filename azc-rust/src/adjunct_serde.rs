//

use crate::{adjunct, arity_serde};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AdjunctSerde {
    #[serde(default)]
    is_entity: bool,

    entities: Vec<AdjuctEntitySerde>,

    #[serde(default)]
    arity: arity_serde::ArityConstraintSerde,
}

impl Into<adjunct::Adjunct> for AdjunctSerde {
    fn into(self) -> adjunct::Adjunct {
        adjunct::Adjunct {
            is_entity: self.is_entity,
            entities: self
                .entities
                .into_iter()
                .map(|x| adjunct::AdjunctEntity::from(x.into()))
                .collect(),
            arity: self.arity.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AdjuctEntitySerde {
    name: String,
}

impl Into<adjunct::AdjunctEntity> for AdjuctEntitySerde {
    fn into(self) -> adjunct::AdjunctEntity {
        adjunct::AdjunctEntity { name: self.name }
    }
}
