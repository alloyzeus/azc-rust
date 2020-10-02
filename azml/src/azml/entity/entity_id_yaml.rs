//

use std::convert;

use crate::azml::{
    entity::{entity_id, entity_id_integer, entity_id_integer_yaml},
    yaml,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EntityIdYaml {
    pub kind: String,
    pub parameters: yaml::Value,
}

impl convert::TryFrom<EntityIdYaml> for entity_id::EntityId {
    type Error = yaml::Error;

    fn try_from(x: EntityIdYaml) -> Result<Self, Self::Error> {
        if x.parameters.is_null() {
            Err(yaml::Error::Msg("Missing definition parameters".to_owned()))
        } else {
            match x.kind.as_str() {
                "integer" => {
                    let def: entity_id_integer_yaml::EntityIdIntegerYaml =
                        yaml::from_value(x.parameters)?;
                    Ok(entity_id::EntityId {
                        definition: Box::new(entity_id_integer::EntityIdInteger::try_from(def)?),
                    })
                }
                _ => Err(yaml::Error::Msg(format!(
                    "Unrecognized entity ID kind `{}`",
                    x.kind
                ))),
            }
        }
    }
}
