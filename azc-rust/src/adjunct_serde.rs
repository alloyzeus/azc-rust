//

use crate::adjunct;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AdjunctSerde {
    #[serde(default)]
    is_entity: bool,

    entities: Vec<String>,
}

impl Into<adjunct::Adjunct> for AdjunctSerde {
    fn into(self) -> adjunct::Adjunct {
        adjunct::Adjunct {
            is_entity: self.is_entity,
            entities: self.entities,
        }
    }
}
