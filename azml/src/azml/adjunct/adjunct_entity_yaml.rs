//

use std::convert::{self, TryInto};

use crate::azml::{
    abstract_yaml, adjunct::adjunct_entity, attribute, attribute_yaml, eid_yaml, ref_key_yaml, yaml,
};

//----

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctEntityYaml {
    #[serde(default)]
    ordering: String,

    id: AdjunctEntityIdYaml,

    #[serde(default)]
    ref_key: ref_key_yaml::RefKeyYaml,

    #[serde(default)]
    implements: abstract_yaml::AbstractImplementationYaml,

    #[serde(default)]
    scope: String,

    #[serde(default)]
    attributes: Vec<attribute_yaml::AttributeYaml>,
}

impl convert::TryFrom<AdjunctEntityYaml> for adjunct_entity::AdjunctEntity {
    type Error = yaml::Error;

    fn try_from(x: AdjunctEntityYaml) -> Result<Self, Self::Error> {
        Ok(adjunct_entity::AdjunctEntity {
            ordering: x.ordering.try_into()?,
            id: x.id.try_into()?,
            ref_key: x.ref_key.try_into()?,
            implements: x.implements.try_into()?,
            scope: x.scope.try_into()?,
            attributes: x
                .attributes
                .iter()
                .map(|attr| attribute::Attribute::try_from(attr).unwrap())
                .collect(),
        })
    }
}

//----

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctEntityIdYaml {
    pub kind: String,
    pub parameters: yaml::Value,
}

impl convert::TryFrom<AdjunctEntityIdYaml> for adjunct_entity::AdjunctEntityId {
    type Error = yaml::Error;

    fn try_from(x: AdjunctEntityIdYaml) -> Result<Self, Self::Error> {
        if x.parameters.is_null() {
            Err(yaml::Error::Msg("Missing definition parameters".to_owned()))
        } else {
            match x.kind.as_str() {
                "integer" => {
                    let def: AdjunctEntityIdIntegerYaml = yaml::from_value(x.parameters)?;
                    Ok(adjunct_entity::AdjunctEntityId {
                        definition: Box::new(adjunct_entity::AdjunctEntityIdInteger::try_from(
                            def,
                        )?),
                    })
                }
                _ => Err(yaml::Error::Msg(format!(
                    "Unrecognized entity ID kind `{}`",
                    x.kind
                ))),
            }
        }
    }
}

//----

pub type AdjunctEntityIdIntegerYaml = eid_yaml::IntegerIdYaml;
