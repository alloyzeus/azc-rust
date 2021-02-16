//

use std::convert::{self, TryInto};

use crate::azml::{abstract_, yaml};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AbstractImplementationYaml {
    kind: String,

    #[serde(default)]
    attributes: Vec<AbstractImplementationAttributeYaml>,
}

impl Default for AbstractImplementationYaml {
    fn default() -> AbstractImplementationYaml {
        AbstractImplementationYaml {
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
        Ok(abstract_::AbstractImplementation {
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
        Ok(abstract_::AbstractImplementationAttribute {
            identifier: x.identifier.to_owned(),
            kind: (&x.kind).into(),
        })
    }
}

//endregion
