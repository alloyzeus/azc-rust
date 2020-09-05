//

use std::{convert, convert::TryInto};

use serde::{Deserialize, Serialize};

use crate::{adjunct, base::arity_serde, base::azml};

#[derive(Serialize, Deserialize)]
pub struct AdjunctSerde {
    #[serde(default)]
    kind: String,

    hosts: Vec<AdjunctHostSerde>,

    #[serde(default)]
    arity: arity_serde::ArityConstraintSerde,

    //TODO: required
    #[serde(default)]
    parameters: azml::Value,
}

impl convert::TryFrom<AdjunctSerde> for adjunct::Adjunct {
    type Error = azml::Error;

    fn try_from(x: AdjunctSerde) -> Result<Self, Self::Error> {
        match x.kind.as_str() {
            "entity" => {
                let params: Option<AdjunctEntitySerde> = azml::from_value(x.parameters)?;
                Ok(adjunct::Adjunct {
                    hosts: x.hosts.into_iter().map(|x| x.into()).collect(),
                    arity: x.arity.into(),
                    parameters: if let Some(p) = params {
                        Some(Box::new(adjunct::AdjunctEntity::from(p)))
                    } else {
                        None
                    },
                })
            }
            _ => {
                //TODO: parameters
                Ok(adjunct::Adjunct {
                    hosts: x.hosts.into_iter().map(|x| x.into()).collect(),
                    arity: x.arity.into(),
                    parameters: None,
                })
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
    #[serde(default)]
    ordering: String,
}

impl From<AdjunctEntitySerde> for adjunct::AdjunctEntity {
    fn from(x: AdjunctEntitySerde) -> adjunct::AdjunctEntity {
        adjunct::AdjunctEntity {
            ordering: x.ordering.try_into().unwrap_or_default(),
        }
    }
}
