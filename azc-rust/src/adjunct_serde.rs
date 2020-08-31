//

use std::convert::TryInto;

use serde::{Deserialize, Serialize};

use crate::{adjunct, base::arity_serde};

#[derive(Serialize, Deserialize)]
pub struct AdjunctSerde {
    #[serde(default)]
    kind: String,

    hosts: Vec<AdjunctHostSerde>,

    #[serde(default)]
    arity: arity_serde::ArityConstraintSerde,

    //TODO: required
    #[serde(default)]
    parameters: serde_yaml::Value,
}

impl From<AdjunctSerde> for adjunct::Adjunct {
    fn from(x: AdjunctSerde) -> adjunct::Adjunct {
        match x.kind.as_str() {
            "entity" => {
                let params: Option<AdjunctEntitySerde> = if x.parameters.is_mapping() {
                    serde_yaml::from_value(x.parameters).unwrap_or(None)
                } else {
                    None
                };
                adjunct::Adjunct {
                    kind: adjunct::AdjunctKind::Entity,
                    hosts: x.hosts.into_iter().map(|x| x.into()).collect(),
                    arity: x.arity.into(),
                    parameters: if let Some(p) = params {
                        Some(Box::new(adjunct::AdjunctEntityDefinition::from(p)))
                    } else {
                        None
                    },
                }
            }
            _ => {
                //TODO: parameters
                adjunct::Adjunct {
                    kind: adjunct::AdjunctKind::ValueObject,
                    hosts: x.hosts.into_iter().map(|x| x.into()).collect(),
                    arity: x.arity.into(),
                    parameters: None,
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AdjunctHostSerde {
    name: String,
}

impl From<AdjunctHostSerde> for adjunct::AdjunctHost {
    fn from(x: AdjunctHostSerde) -> adjunct::AdjunctHost {
        adjunct::AdjunctHost { name: x.name }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AdjunctEntitySerde {
    ordering: String,
}

impl From<AdjunctEntitySerde> for adjunct::AdjunctEntityDefinition {
    fn from(x: AdjunctEntitySerde) -> adjunct::AdjunctEntityDefinition {
        adjunct::AdjunctEntityDefinition {
            ordering: x.ordering.try_into().unwrap_or_default(),
        }
    }
}
