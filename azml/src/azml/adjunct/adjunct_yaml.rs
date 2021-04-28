//

use std::{convert, convert::TryInto};

use crate::azml::{
    adjunct::{
        adjunct, adjunct_entity, adjunct_entity_yaml, adjunct_value_object,
        adjunct_value_object_yaml,
    },
    cardinality_yaml, yaml,
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
        match x.kind.as_str() {
            "entity" => {
                let def: adjunct_entity_yaml::AdjunctEntityYaml = yaml::from_value(x.parameters)?;
                Ok(adjunct::Adjunct {
                    hosts: x.hosts.into_iter().map(|x| x.into()).collect(),
                    cardinality: x.cardinality.into(),
                    definition: Box::new(adjunct_entity::AdjunctEntity::from(def.try_into()?)),
                    name_is_prepared: x.name_is_prepared,
                })
            }
            "value_object" => {
                let def: adjunct_value_object_yaml::AdjunctValueObjectYaml =
                    yaml::from_value(x.parameters)?;
                Ok(adjunct::Adjunct {
                    hosts: x.hosts.into_iter().map(|x| x.into()).collect(),
                    cardinality: x.cardinality.into(),
                    definition: Box::new(adjunct_value_object::AdjunctValueObject::from(
                        def.try_into()?,
                    )),
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
}

impl From<AdjunctHostYaml> for adjunct::AdjunctHost {
    fn from(x: AdjunctHostYaml) -> adjunct::AdjunctHost {
        adjunct::AdjunctHost { kind: x.kind }
    }
}
