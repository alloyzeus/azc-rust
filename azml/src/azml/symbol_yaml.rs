//

use std::convert;

use crate::azml::{
    adjunct::{adjunct, adjunct_yaml},
    entity::{entity, entity_yaml},
    symbol,
    value_object::{value_object, value_object_yaml},
    yaml,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SymbolYaml {
    identifier: String,
    kind: String,

    //TODO: required
    #[serde(default)]
    parameters: yaml::Value,

    #[serde(default)]
    documentation: String,
}

impl convert::TryFrom<SymbolYaml> for symbol::Symbol {
    type Error = yaml::Error;

    fn try_from(x: SymbolYaml) -> Result<Self, Self::Error> {
        match x.kind.as_str() {
            "entity" => {
                let params: Option<entity_yaml::EntityYaml> = yaml::from_value(x.parameters)?;
                match params {
                    Some(p) => Ok(symbol::Symbol {
                        identifier: x.identifier,
                        definition: Box::new(entity::Entity::try_from(p)?),
                        documentation: x.documentation,
                    }),
                    None => Err(yaml::Error::Msg("Missing definition".to_owned())),
                }
            }
            "adjunct" => {
                let params: Option<adjunct_yaml::AdjunctYaml> = yaml::from_value(x.parameters)?;
                match params {
                    Some(p) => Ok(symbol::Symbol {
                        identifier: x.identifier,
                        definition: Box::new(adjunct::Adjunct::try_from(p)?),
                        documentation: x.documentation,
                    }),
                    None => Err(yaml::Error::Msg("Missing definition".to_owned())),
                }
            }
            "value_object" => {
                let params: Option<value_object_yaml::ValueObjectYaml> =
                    yaml::from_value(x.parameters)?;
                match params {
                    Some(p) => Ok(symbol::Symbol {
                        identifier: x.identifier,
                        definition: Box::new(value_object::ValueObject::try_from(p)?),
                        documentation: x.documentation,
                    }),
                    None => Err(yaml::Error::Msg("Missing definition".to_owned())),
                }
            }
            _ => Err(yaml::Error::Msg(format!(
                r#"Unrecognized symbol kind `{}`"#,
                x.kind
            ))),
        }
    }
}
