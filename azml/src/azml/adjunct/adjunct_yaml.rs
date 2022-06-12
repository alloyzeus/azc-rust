//

use std::{convert, convert::TryInto};

use crate::azml::{cardinality_yaml, symbol, yaml};

use super::{adjunct,
    adjunct_entity::{self, adjunct_entity_yaml},
    adjunct_value::{self, adjunct_value_yaml}};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctYaml {
    #[serde(default)]
    kind: String,

    hosts: Vec<AdjunctHostYaml>,

    #[serde(default)]
    cardinality: cardinality_yaml::CardinalityConstraintYaml,

    parameters: yaml::Value,

    #[serde(default)]
    name_is_prepared: bool,
}

impl convert::TryFrom<AdjunctYaml> for adjunct::Adjunct {
    type Error = yaml::Error;

    fn try_from(x: AdjunctYaml) -> Result<Self, Self::Error> {
        let hosts = x.hosts.iter().map(|h| h.into()).collect();
        match x.kind.as_str() {
            "" => Ok(adjunct::Adjunct {
                hosts: hosts,
                cardinality: x.cardinality.into(),
                definition: Box::new(adjunct::AdjunctNone {}),
                name_is_prepared: x.name_is_prepared,
            }),
            "none" => Ok(adjunct::Adjunct {
                hosts: hosts,
                cardinality: x.cardinality.into(),
                definition: Box::new(adjunct::AdjunctNone {}),
                name_is_prepared: x.name_is_prepared,
            }),
            "entity" => {
                let def: adjunct_entity_yaml::AdjunctEntityYaml = yaml::from_value(x.parameters)?;
                Ok(adjunct::Adjunct {
                    hosts: hosts,
                    cardinality: x.cardinality.into(),
                    definition: Box::new(adjunct_entity::AdjunctEntity::from(def.try_into()?)),
                    name_is_prepared: x.name_is_prepared,
                })
            }
            "value" => {
                let def: adjunct_value_yaml::AdjunctPrimeYaml = yaml::from_value(x.parameters)?;
                Ok(adjunct::Adjunct {
                    hosts: hosts,
                    cardinality: x.cardinality.into(),
                    definition: Box::new(adjunct_value::AdjunctPrime::from(def.try_into()?)),
                    name_is_prepared: x.name_is_prepared,
                })
            }
            _ => Err(yaml::Error::Msg(format!(
                "Unrecognized adjunct kind `{}`",
                x.kind
            ))),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctHostYaml {
    kind: String,

    #[serde(default)]
    name: String,
}

impl From<AdjunctHostYaml> for adjunct::AdjunctHost {
    fn from(x: AdjunctHostYaml) -> adjunct::AdjunctHost {
        (&x).into()
    }
}

impl From<&AdjunctHostYaml> for adjunct::AdjunctHost {
    fn from(x: &AdjunctHostYaml) -> adjunct::AdjunctHost {
        adjunct::AdjunctHost {
            kind: symbol::SymbolRef::from(x.kind.to_owned()),
            name: x.name.to_owned(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctNoneYaml {}

impl convert::TryFrom<AdjunctNoneYaml> for adjunct::AdjunctNone {
    type Error = yaml::Error;

    fn try_from(_x: AdjunctNoneYaml) -> Result<Self, Self::Error> {
        Ok(adjunct::AdjunctNone {})
    }
}
