//

use std::convert;

use crate::azml::{
    adjunct::{adjunct, adjunct_yaml},
    entity::{
        abstract_, abstract_yaml,
        root_entity::{root_entity, root_entity_yaml},
    },
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

impl convert::TryFrom<&SymbolYaml> for symbol::Symbol {
    type Error = yaml::Error;

    fn try_from(x: &SymbolYaml) -> Result<Self, Self::Error> {
        match x.kind.as_str() {
            "entity" => {
                let def: root_entity_yaml::RootEntityYaml = yaml::from_value(x.parameters.clone())?;
                Ok(Self {
                    identifier: x.identifier.to_owned(),
                    definition: Box::new(root_entity::RootEntity::try_from(def)?),
                    documentation: x.documentation.to_owned(),
                })
            }
            "adjunct" => {
                let def: adjunct_yaml::AdjunctYaml = yaml::from_value(x.parameters.clone())?;
                Ok(Self {
                    identifier: x.identifier.to_owned(),
                    definition: Box::new(adjunct::Adjunct::try_from(def)?),
                    documentation: x.documentation.to_owned(),
                })
            }
            "value_object" => {
                let def: value_object_yaml::ValueObjectYaml =
                    yaml::from_value(x.parameters.clone())?;
                Ok(Self {
                    identifier: x.identifier.to_owned(),
                    definition: Box::new(value_object::ValueObject::try_from(def)?),
                    documentation: x.documentation.to_owned(),
                })
            }
            "abstract" => {
                let def: abstract_yaml::AbstractYaml = yaml::from_value(x.parameters.clone())?;
                Ok(Self {
                    identifier: x.identifier.to_owned(),
                    definition: Box::new(abstract_::Abstract::try_from(def)?),
                    documentation: x.documentation.to_owned(),
                })
            }
            _ => Err(yaml::Error::Msg(format!(
                "Unrecognized symbol kind `{}`",
                x.kind
            ))),
        }
    }
}
