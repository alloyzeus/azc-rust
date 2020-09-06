//

use serde::{Deserialize, Serialize};
use std::convert;

use crate::{
    azyaml, entity::entity_id, entity::entity_id_integer, entity::entity_id_integer_serde,
};

#[derive(Deserialize, Serialize)]
pub struct EntityIdSerde {
    pub kind: String,
    pub parameters: azyaml::Value,
}

impl convert::TryFrom<EntityIdSerde> for entity_id::EntityId {
    type Error = azyaml::Error;

    fn try_from(x: EntityIdSerde) -> Result<Self, Self::Error> {
        match x.kind.as_str() {
            "integer" => {
                let params: Option<entity_id_integer_serde::EntityIdIntegerSerde> =
                    azyaml::from_value(x.parameters)?;
                Ok(entity_id::EntityId {
                    parameters: if let Some(p) = params {
                        Some(Box::new(entity_id_integer::EntityIdInteger::try_from(p)?))
                    } else {
                        None
                    },
                })
            }
            _ => Err(azyaml::Error::Msg(format!(
                r#"Unrecognized symbol kind `{}`"#,
                x.kind
            ))),
        }
    }
}
