//

use serde::{Deserialize, Serialize};
use std::convert;

use crate::{adjunct, adjunct_serde, azyaml, entity::entity, entity::entity_serde, symbol};

#[derive(Serialize, Deserialize)]
pub struct SymbolSerde {
    identifier: String,
    kind: String,

    //TODO: required
    #[serde(default)]
    parameters: azyaml::Value,
}

impl convert::TryFrom<SymbolSerde> for symbol::Symbol {
    type Error = azyaml::Error;

    fn try_from(x: SymbolSerde) -> Result<Self, Self::Error> {
        match x.kind.as_str() {
            "entity" => {
                let params: Option<entity_serde::EntitySerde> = azyaml::from_value(x.parameters)?;
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
                let params: Option<adjunct_serde::AdjunctSerde> = azyaml::from_value(x.parameters)?;
                Ok(symbol::Symbol {
                    identifier: x.identifier,
                    parameters: if let Some(p) = params {
                        Some(Box::new(adjunct::Adjunct::try_from(p)?))
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
