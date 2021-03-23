//

use std::convert;

use crate::azml::{
    entity::{entity_id_num, entity_id_num_integer, entity_id_num_integer_yaml},
    yaml,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EntityIdNumYaml {
    pub kind: String,
    pub parameters: yaml::Value,
}

impl convert::TryFrom<EntityIdNumYaml> for entity_id_num::EntityIdNum {
    type Error = yaml::Error;

    fn try_from(x: EntityIdNumYaml) -> Result<Self, Self::Error> {
        if x.parameters.is_null() {
            Err(yaml::Error::Msg("Missing definition parameters".to_owned()))
        } else {
            match x.kind.as_str() {
                "integer" => {
                    let def: entity_id_num_integer_yaml::EntityIdNumIntegerYaml =
                        yaml::from_value(x.parameters)?;
                    Ok(entity_id_num::EntityIdNum {
                        definition: Box::new(entity_id_num_integer::EntityIdNumInteger::try_from(
                            def,
                        )?),
                    })
                }
                _ => Err(yaml::Error::Msg(format!(
                    "Unrecognized entity id_num kind `{}`",
                    x.kind
                ))),
            }
        }
    }
}
