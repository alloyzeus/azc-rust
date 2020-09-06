//

use std::convert;

use crate::azml::{
    entity::entity_id, entity::entity_id_integer, entity::entity_id_integer_yaml, yaml,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EntityIdYaml {
    pub kind: String,
    pub parameters: yaml::Value,
}

impl convert::TryFrom<EntityIdYaml> for entity_id::EntityId {
    type Error = yaml::Error;

    fn try_from(x: EntityIdYaml) -> Result<Self, Self::Error> {
        match x.kind.as_str() {
            "integer" => {
                let params: Option<entity_id_integer_yaml::EntityIdIntegerYaml> =
                    yaml::from_value(x.parameters)?;
                Ok(entity_id::EntityId {
                    parameters: if let Some(p) = params {
                        Some(Box::new(entity_id_integer::EntityIdInteger::try_from(p)?))
                    } else {
                        None
                    },
                })
            }
            _ => Err(yaml::Error::Msg(format!(
                r#"Unrecognized symbol kind `{}`"#,
                x.kind
            ))),
        }
    }
}
