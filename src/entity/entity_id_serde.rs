//

use serde::{Deserialize, Serialize};
use std::convert;

use crate::{
    base::azml, entity::entity_id, entity::entity_id_integer, entity::entity_id_integer_serde,
};

#[derive(Deserialize, Serialize)]
pub struct EntityIdDefinitionSerde {
    pub kind: String,
    pub parameters: azml::Value,
}

impl convert::TryFrom<EntityIdDefinitionSerde> for entity_id::EntityIdDefinition {
    type Error = azml::Error;

    fn try_from(x: EntityIdDefinitionSerde) -> Result<Self, Self::Error> {
        match x.kind.as_str() {
            "integer" => {
                let params: Option<entity_id_integer_serde::EntityIdIntegerDefinitionSerde> =
                    azml::from_value(x.parameters)?;
                Ok(entity_id::EntityIdDefinition {
                    kind: x.kind,
                    parameters: if let Some(p) = params {
                        Some(Box::new(entity_id_integer::EntityIdInteger::try_from(p)?))
                    } else {
                        None
                    },
                })
            }
            _ => Err(azml::Error::Msg(format!(
                r#"Unrecognized symbol kind `{}`"#,
                x.kind
            ))),
        }
    }
}
