//

use std::convert::{self, TryInto};

use crate::azml::{
    adjunct::adjunct_value,
    entity::{abstract_, abstract_yaml},
    yaml,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctValueYaml {
    #[serde(default)]
    documentation: String,

    #[serde(default)]
    implements: Vec<abstract_yaml::AbstractImplementationYaml>,

    kind: String,
}

impl convert::TryFrom<&AdjunctValueYaml> for adjunct_value::AdjunctValue {
    type Error = yaml::Error;

    fn try_from(x: &AdjunctValueYaml) -> Result<Self, Self::Error> {
        Ok(Self {
            documentation: x.documentation.to_owned(),
            implements: x
                .implements
                .iter()
                .map(|x| abstract_::AbstractImplementation::try_from(x))
                .collect::<Result<Vec<abstract_::AbstractImplementation>, _>>()?,
            kind: x.kind.to_owned(),
        })
    }
}

impl convert::TryFrom<AdjunctValueYaml> for adjunct_value::AdjunctValue {
    type Error = yaml::Error;

    fn try_from(x: AdjunctValueYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}
