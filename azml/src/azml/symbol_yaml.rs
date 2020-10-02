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

    parameters: yaml::Value,

    #[serde(default)]
    documentation: String,
}

impl convert::TryFrom<SymbolYaml> for symbol::Symbol {
    type Error = yaml::Error;

    fn try_from(x: SymbolYaml) -> Result<Self, Self::Error> {
        match x.kind.as_str() {
            "entity" => {
                let def: entity_yaml::EntityYaml = yaml::from_value(x.parameters)?;
                Ok(symbol::Symbol {
                    identifier: x.identifier,
                    definition: Box::new(entity::Entity::try_from(def)?),
                    documentation: x.documentation,
                })
            }
            "adjunct" => {
                let def: adjunct_yaml::AdjunctYaml = yaml::from_value(x.parameters)?;
                Ok(symbol::Symbol {
                    identifier: x.identifier,
                    definition: Box::new(adjunct::Adjunct::try_from(def)?),
                    documentation: x.documentation,
                })
            }
            "value_object" => {
                let def: value_object_yaml::ValueObjectYaml = yaml::from_value(x.parameters)?;
                Ok(symbol::Symbol {
                    identifier: x.identifier,
                    definition: Box::new(value_object::ValueObject::try_from(def)?),
                    documentation: x.documentation,
                })
            }
            _ => Err(yaml::Error::Msg(format!(
                "Unrecognized symbol kind `{}`",
                x.kind
            ))),
        }
    }
}
