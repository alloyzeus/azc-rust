//

use std::{convert, convert::TryInto};

use crate::azml::{cardinality_yaml, symbol, yaml};

use super::{
    adjunct,
    adjunct_entity::{self, adjunct_entity_yaml},
    adjunct_prime::{self, adjunct_prime_yaml},
    adjunct_value::{self, adjunct_value_yaml},
};

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
            "" => Ok(Self {
                hosts,
                cardinality: x.cardinality.into(),
                definition: Box::new(adjunct::AdjunctNone {}),
                name_is_prepared: x.name_is_prepared,
            }),
            "none" => Ok(Self {
                hosts,
                cardinality: x.cardinality.into(),
                definition: Box::new(adjunct::AdjunctNone {}),
                name_is_prepared: x.name_is_prepared,
            }),
            "entity" => {
                let def: adjunct_entity_yaml::AdjunctEntityYaml = yaml::from_value(x.parameters)?;
                Ok(Self {
                    hosts,
                    cardinality: x.cardinality.into(),
                    definition: Box::new(adjunct_entity::AdjunctEntity::from(def.try_into()?)),
                    name_is_prepared: x.name_is_prepared,
                })
            }
            "prime" => {
                let def: adjunct_prime_yaml::AdjunctPrimeYaml = yaml::from_value(x.parameters)?;
                Ok(Self {
                    hosts,
                    cardinality: x.cardinality.into(),
                    definition: Box::new(adjunct_prime::AdjunctPrime::from(def.try_into()?)),
                    name_is_prepared: x.name_is_prepared,
                })
            }
            "value" => {
                let def: adjunct_value_yaml::AdjunctValueYaml = yaml::from_value(x.parameters)?;
                Ok(Self {
                    hosts,
                    cardinality: x.cardinality.into(),
                    definition: Box::new(adjunct_value::AdjunctValue::from(def.try_into()?)),
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

    #[serde(default)]
    cardinality: cardinality_yaml::CardinalityConstraintYaml,
    #[serde(default)]
    inverse_cardinality: cardinality_yaml::CardinalityConstraintYaml,
}

impl From<AdjunctHostYaml> for adjunct::AdjunctHost {
    fn from(x: AdjunctHostYaml) -> Self {
        (&x).into()
    }
}

impl From<&AdjunctHostYaml> for adjunct::AdjunctHost {
    fn from(x: &AdjunctHostYaml) -> Self {
        Self {
            kind: symbol::SymbolRef::from(x.kind.to_owned()),
            name: x.name.to_owned(),
            cardinality: (&x.cardinality).into(),
            inverse_cardinality: (&x.inverse_cardinality).into(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctNoneYaml {}

impl convert::TryFrom<AdjunctNoneYaml> for adjunct::AdjunctNone {
    type Error = yaml::Error;

    fn try_from(_x: AdjunctNoneYaml) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
