//

use std::{convert, convert::TryInto};

use crate::azml::{adjunct::adjunct, arity_yaml, yaml};

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
                match params {
                    Some(p) => Ok(adjunct::Adjunct {
                        hosts: x.hosts.into_iter().map(|x| x.into()).collect(),
                        arity: x.arity.into(),
                        parameters: Box::new(adjunct::AdjunctEntity::from(p)),
                    }),
                    None => Err(yaml::Error::Msg("Missing parameters".to_owned())),
                }
            }
            "value-object" => {
                let params: Option<AdjunctValueObjectYaml> = yaml::from_value(x.parameters)?;
                match params {
                    Some(p) => Ok(adjunct::Adjunct {
                        hosts: x.hosts.into_iter().map(|x| x.into()).collect(),
                        arity: x.arity.into(),
                        parameters: Box::new(adjunct::AdjunctValueObject::from(p.try_into()?)),
                    }),
                    None => Err(yaml::Error::Msg("Missing parameters".to_owned())),
                }
            }
            _ => {
                Err(yaml::Error::Msg(format!(
                    r#"Unrecognized symbol kind `{}`"#,
                    x.kind
                )))
                // //TODO: parameters
                // Ok(adjunct::Adjunct {
                //     hosts: x.hosts.into_iter().map(|x| x.into()).collect(),
                //     arity: x.arity.into(),
                //     parameters: None,
                // })
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

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctValueObjectYaml {}

impl convert::TryFrom<AdjunctValueObjectYaml> for adjunct::AdjunctValueObject {
    type Error = yaml::Error;

    fn try_from(_x: AdjunctValueObjectYaml) -> Result<Self, Self::Error> {
        Ok(adjunct::AdjunctValueObject {})
    }
}
