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

impl From<AdjunctSerde> for adjunct::Adjunct {
    fn from(x: AdjunctSerde) -> adjunct::Adjunct {
        adjunct::Adjunct {
            is_entity: x.is_entity,
            entities: x.entities.into_iter().map(|x| x.into()).collect(),
            arity: x.arity.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AdjuctEntitySerde {
    name: String,
}

impl From<AdjuctEntitySerde> for adjunct::AdjunctEntity {
    fn from(x: AdjuctEntitySerde) -> adjunct::AdjunctEntity {
        adjunct::AdjunctEntity { name: x.name }
    }
}
