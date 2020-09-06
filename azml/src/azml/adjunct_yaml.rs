//

use std::{convert, convert::TryInto};

use crate::azml::{adjunct, arity_yaml, yaml};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctYaml {
    #[serde(default)]
    kind: String,

    hosts: Vec<AdjunctHostYaml>,

    #[serde(default)]
    arity: arity_yaml::ArityConstraintYaml,

    //TODO: required
    #[serde(default)]
    parameters: yaml::Value,
}

impl convert::TryFrom<AdjunctYaml> for adjunct::Adjunct {
    type Error = yaml::Error;

    fn try_from(x: AdjunctYaml) -> Result<Self, Self::Error> {
        match x.kind.as_str() {
            "entity" => {
                let params: Option<AdjunctEntityYaml> = yaml::from_value(x.parameters)?;
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

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctHostYaml {
    name: String,
}

impl From<AdjunctHostYaml> for adjunct::AdjunctHost {
    fn from(x: AdjunctHostYaml) -> adjunct::AdjunctHost {
        adjunct::AdjunctHost { name: x.name }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctEntityYaml {
    #[serde(default)]
    ordering: String,
}

impl From<AdjunctEntityYaml> for adjunct::AdjunctEntity {
    fn from(x: AdjunctEntityYaml) -> adjunct::AdjunctEntity {
        adjunct::AdjunctEntity {
            ordering: x.ordering.try_into().unwrap_or_default(),
        }
    }
}
