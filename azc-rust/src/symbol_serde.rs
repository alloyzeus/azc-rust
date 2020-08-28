//

use crate::{adjunct, adjunct_serde, entity, entity_serde, symbol};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SymbolSerde {
    name: String,
    kind: String,

    #[serde(default)]
    parameters: serde_yaml::Value,
}

impl Into<symbol::Symbol> for SymbolSerde {
    fn into(self) -> symbol::Symbol {
        match self.kind.as_str() {
            "entity" => {
                let params: Option<entity_serde::EntitySerde> = if self.parameters.is_mapping() {
                    serde_yaml::from_value(self.parameters).unwrap()
                } else {
                    None
                };
                symbol::Symbol {
                    name: self.name,
                    kind: self.kind,
                    parameters: if params.is_some() {
                        Some(Box::new(entity::Entity::from(params.unwrap().into())))
                    } else {
                        None
                    },
                }
            }
            "adjunct" => {
                let params: Option<adjunct_serde::AdjunctSerde> = if self.parameters.is_mapping() {
                    serde_yaml::from_value(self.parameters).unwrap()
                } else {
                    None
                };
                symbol::Symbol {
                    name: self.name,
                    kind: self.kind,
                    parameters: if params.is_some() {
                        Some(Box::new(adjunct::Adjunct::from(params.unwrap().into())))
                    } else {
                        None
                    },
                }
            }
            _ => panic!(format!("unrecognized symbol kind: {}", self.kind)),
        }
    }
}
