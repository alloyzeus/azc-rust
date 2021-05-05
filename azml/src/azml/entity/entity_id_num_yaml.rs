//

use std::convert::{self, TryInto};

use crate::azml::{
    entity::{entity_id_num, entity_id_num_integer, entity_id_num_integer_yaml},
    id::ref_key_yaml,
    yaml,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EntityIdYaml {
    pub num: EntityIdNumYaml,

    #[serde(default)]
    pub ref_key: ref_key_yaml::RefKeyYaml,
}

impl convert::TryFrom<&EntityIdYaml> for entity_id_num::EntityId {
    type Error = yaml::Error;

    fn try_from(x: &EntityIdYaml) -> Result<Self, Self::Error> {
        Ok(entity_id_num::EntityId {
            num: (&x.num).try_into()?,
            ref_key: (&x.ref_key).try_into()?,
        })
    }
}

impl convert::TryFrom<EntityIdYaml> for entity_id_num::EntityId {
    type Error = yaml::Error;

    fn try_from(x: EntityIdYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EntityIdNumYaml {
    pub kind: String,
    pub parameters: yaml::Value,
}

impl convert::TryFrom<&EntityIdNumYaml> for entity_id_num::EntityIdNum {
    type Error = yaml::Error;

    fn try_from(x: &EntityIdNumYaml) -> Result<Self, Self::Error> {
        if x.parameters.is_null() {
            Err(yaml::Error::Msg("Missing definition parameters".to_owned()))
        } else {
            match x.kind.as_str() {
                "integer" => {
                    let def: entity_id_num_integer_yaml::EntityIdNumIntegerYaml =
                        yaml::from_value(x.parameters.clone())?;
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

impl convert::TryFrom<EntityIdNumYaml> for entity_id_num::EntityIdNum {
    type Error = yaml::Error;

    fn try_from(x: EntityIdNumYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}
