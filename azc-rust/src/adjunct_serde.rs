//

use crate::{adjunct, arity_serde};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AdjunctSerde {
    #[serde(default)]
    is_entity: bool,

    entities: Vec<String>,

    #[serde(default)]
    arity: arity_serde::ArityConstraintSerde,
}

impl Into<adjunct::Adjunct> for AdjunctSerde {
    fn into(self) -> adjunct::Adjunct {
        adjunct::Adjunct {
            is_entity: self.is_entity,
            entities: self.entities,
            arity: self.arity.into(),
        }
    }
}
