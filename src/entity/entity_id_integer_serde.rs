//

use serde::{Deserialize, Serialize};
use std::convert;

use crate::{base::azml, entity::entity_id_integer};

#[derive(Deserialize, Serialize)]
pub struct EntityIdIntegerSerde {
    space: i8,
}

impl convert::TryFrom<EntityIdIntegerSerde> for entity_id_integer::EntityIdInteger {
    type Error = azml::Error;

    fn try_from(x: EntityIdIntegerSerde) -> Result<Self, Self::Error> {
        Ok(entity_id_integer::EntityIdInteger { space: x.space })
    }
}
