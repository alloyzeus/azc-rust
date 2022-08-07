//

use std::convert::{self, TryInto};

use crate::azml::yaml;

use super::abstract_;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AbstractYaml {
    #[serde(default)]
    documentation: String,

    #[serde(default)]
    singular: bool,

    #[serde(default)]
    attributes: Vec<AbstractAttributeYaml>,
}

impl convert::TryFrom<AbstractYaml> for abstract_::Abstract {
    type Error = yaml::Error;

    fn try_from(x: AbstractYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

impl convert::TryFrom<&AbstractYaml> for abstract_::Abstract {
    type Error = yaml::Error;

    fn try_from(x: &AbstractYaml) -> Result<Self, Self::Error> {
        Ok(Self {
            documentation: x.documentation.to_owned(),
            singular: x.singular,
            attributes: (&x.attributes)
                .iter()
                .map(|x| abstract_::AbstractAttribute::try_from(x))
                .collect::<Result<Vec<abstract_::AbstractAttribute>, _>>()?,
        })
    }
}

//region AbstractAttribute

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AbstractAttributeYaml {
    name: String,
}

impl convert::TryFrom<AbstractAttributeYaml> for abstract_::AbstractAttribute {
    type Error = yaml::Error;

    fn try_from(x: AbstractAttributeYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

impl convert::TryFrom<&AbstractAttributeYaml> for abstract_::AbstractAttribute {
    type Error = yaml::Error;

    fn try_from(x: &AbstractAttributeYaml) -> Result<Self, Self::Error> {
        Ok(Self {
            name: x.name.to_owned(),
        })
    }
}

//endregion

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AbstractImplementationYaml {
    kind: String,

    #[serde(default)]
    attributes: Vec<AbstractImplementationAttributeYaml>,
}

impl Default for AbstractImplementationYaml {
    fn default() -> Self {
        Self {
            kind: "".to_owned(),
            attributes: Vec::new(),
        }
    }
}

impl convert::TryFrom<AbstractImplementationYaml> for abstract_::AbstractImplementation {
    type Error = yaml::Error;

    fn try_from(x: AbstractImplementationYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

impl convert::TryFrom<&AbstractImplementationYaml> for abstract_::AbstractImplementation {
    type Error = yaml::Error;

    fn try_from(x: &AbstractImplementationYaml) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: (&x.kind).into(),
            attributes: x
                .attributes
                .iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<abstract_::AbstractImplementationAttribute>, _>>()?,
        })
    }
}

//region AbstractImplementationAttributeYaml

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AbstractImplementationAttributeYaml {
    identifier: String,

    kind: String,
}

impl convert::TryFrom<AbstractImplementationAttributeYaml>
    for abstract_::AbstractImplementationAttribute
{
    type Error = yaml::Error;

    fn try_from(x: AbstractImplementationAttributeYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

impl convert::TryFrom<&AbstractImplementationAttributeYaml>
    for abstract_::AbstractImplementationAttribute
{
    type Error = yaml::Error;

    fn try_from(x: &AbstractImplementationAttributeYaml) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: x.identifier.to_owned(),
            kind: (&x.kind).into(),
        })
    }
}

//endregion
