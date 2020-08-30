//

use crate::{adjunct, adjunct_serde, entity, entity_serde, symbol, symbol_kind};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SymbolSerde {
    identifier: String,
    kind: String,

    //TODO: required
    #[serde(default)]
    parameters: serde_yaml::Value,
}

impl From<SymbolSerde> for symbol::Symbol {
    fn from(x: SymbolSerde) -> symbol::Symbol {
        match x.kind.as_str() {
            "entity" => {
                let params: Option<entity_serde::EntitySerde> = if x.parameters.is_mapping() {
                    serde_yaml::from_value(x.parameters).unwrap_or(None)
                } else {
                    None
                };
                symbol::Symbol {
                    identifier: x.identifier,
                    kind: symbol_kind::SymbolKind::Entity,
                    parameters: if let Some(p) = params {
                        Some(Box::new(entity::Entity::from(p)))
                    } else {
                        None
                    },
                }
            }
            "adjunct" => {
                let params: Option<adjunct_serde::AdjunctSerde> = if x.parameters.is_mapping() {
                    serde_yaml::from_value(x.parameters).unwrap_or(None)
                } else {
                    None
                };
                symbol::Symbol {
                    identifier: x.identifier,
                    kind: symbol_kind::SymbolKind::Adjunct,
                    parameters: if let Some(p) = params {
                        Some(Box::new(adjunct::Adjunct::from(p)))
                    } else {
                        None
                    },
                }
            }
            _ => panic!(format!("unrecognized symbol kind: {}", x.kind)),
        }
    }
}
