//

use std::convert::{self, TryInto};

use crate::azml::{
    adjunct::adjunct_entity,
    attribute, attribute_yaml,
    entity::{
        abstract_, abstract_yaml,
        id::{id_num_yaml, ref_key_yaml},
        lifecycle::lifecycle_yaml,
    },
    yaml,
};

//----

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctEntityYaml {
    id: AdjunctEntityIdYaml,

    #[serde(default)]
    ordering: String,

    #[serde(default)]
    implements: Vec<abstract_yaml::AbstractImplementationYaml>,

    lifecycle: lifecycle_yaml::LifecycleYaml,

    #[serde(default)]
    scope: String,

    #[serde(default)]
    attributes: Vec<attribute_yaml::AttributeYaml>,
}

impl convert::TryFrom<AdjunctEntityYaml> for adjunct_entity::AdjunctEntity {
    type Error = yaml::Error;

    fn try_from(x: AdjunctEntityYaml) -> Result<Self, Self::Error> {
        Ok(Self {
            id: x.id.try_into()?,
            ordering: x.ordering.try_into()?,
            implements: x
                .implements
                .iter()
                .map(|x| abstract_::AbstractImplementation::try_from(x))
                .collect::<Result<Vec<abstract_::AbstractImplementation>, _>>()?,
            lifecycle: x.lifecycle.try_into()?,
            scope: x.scope.try_into()?,
            attributes: x
                .attributes
                .iter()
                .map(|attr| attribute::Attribute::try_from(attr))
                .collect::<Result<Vec<attribute::Attribute>, _>>()?,
        })
    }
}

//----

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctEntityIdYaml {
    pub num: AdjunctEntityIdNumYaml,

    #[serde(default)]
    pub ref_key: ref_key_yaml::RefKeyYaml,
}

impl convert::TryFrom<&AdjunctEntityIdYaml> for adjunct_entity::AdjunctEntityId {
    type Error = yaml::Error;

    fn try_from(x: &AdjunctEntityIdYaml) -> Result<Self, Self::Error> {
        Ok(Self {
            num: (&x.num).try_into()?,
            ref_key: (&x.ref_key).try_into()?,
        })
    }
}

impl convert::TryFrom<AdjunctEntityIdYaml> for adjunct_entity::AdjunctEntityId {
    type Error = yaml::Error;

    fn try_from(x: AdjunctEntityIdYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//----

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctEntityIdNumYaml {
    pub kind: String,
    pub parameters: yaml::Value,
}

impl convert::TryFrom<&AdjunctEntityIdNumYaml> for adjunct_entity::AdjunctEntityIdNum {
    type Error = yaml::Error;

    fn try_from(x: &AdjunctEntityIdNumYaml) -> Result<Self, Self::Error> {
        if x.parameters.is_null() {
            Err(yaml::Error::Msg("Missing definition parameters".to_owned()))
        } else {
            match x.kind.as_str() {
                "integer" => {
                    let def: AdjunctEntityIdNumIntegerYaml =
                        yaml::from_value(x.parameters.clone())?;
                    Ok(Self {
                        definition: Box::new(adjunct_entity::AdjunctEntityIdNumInteger::try_from(
                            def,
                        )?),
                    })
                }
                _ => Err(yaml::Error::Msg(format!(
                    "Unrecognized entity id_num kind `{}`",
                    x.kind
                ))),
            }
        }
    }
}

impl convert::TryFrom<AdjunctEntityIdNumYaml> for adjunct_entity::AdjunctEntityIdNum {
    type Error = yaml::Error;

    fn try_from(x: AdjunctEntityIdNumYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//----

pub type AdjunctEntityIdNumIntegerYaml = id_num_yaml::IntegerIdNumYaml;
