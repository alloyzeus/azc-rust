//

use std::convert;

use crate::azml::{entity::entity_id_integer, yaml};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EntityIdIntegerYaml {
    space: i8,
}

impl convert::TryFrom<EntityIdIntegerYaml> for entity_id_integer::EntityIdInteger {
    type Error = yaml::Error;

    fn try_from(x: EntityIdIntegerYaml) -> Result<Self, Self::Error> {
        Ok(entity_id_integer::EntityIdInteger { space: x.space })
    }
}
