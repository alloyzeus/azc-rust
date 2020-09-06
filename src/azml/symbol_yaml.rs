//

use std::convert;

use crate::azml::{adjunct, adjunct_yaml, entity::entity, entity::entity_yaml, symbol, yaml};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SymbolYaml {
    identifier: String,
    kind: String,

    //TODO: required
    #[serde(default)]
    parameters: yaml::Value,
}

impl convert::TryFrom<SymbolYaml> for symbol::Symbol {
    type Error = yaml::Error;

    fn try_from(x: SymbolYaml) -> Result<Self, Self::Error> {
        match x.kind.as_str() {
            "entity" => {
                let params: Option<entity_yaml::EntityYaml> = yaml::from_value(x.parameters)?;
                Ok(symbol::Symbol {
                    identifier: x.identifier,
                    parameters: if let Some(p) = params {
                        Some(Box::new(entity::Entity::try_from(p)?))
                    } else {
                        None
                    },
                })
            }
            "adjunct" => {
                let params: Option<adjunct_yaml::AdjunctYaml> = yaml::from_value(x.parameters)?;
                Ok(symbol::Symbol {
                    identifier: x.identifier,
                    parameters: if let Some(p) = params {
                        Some(Box::new(adjunct::Adjunct::try_from(p)?))
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
