//

use std::convert::{self, TryInto};

use crate::azml::{eid, yaml};

//region IntegerIdYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct IntegerIdYaml {
    bits: i8,

    #[serde(default)]
    flags: Vec<IntegerIdBitFlagYaml>,
}

impl convert::TryFrom<IntegerIdYaml> for eid::IntegerId {
    type Error = yaml::Error;

    fn try_from(x: IntegerIdYaml) -> Result<Self, Self::Error> {
        Ok(eid::IntegerId {
            bits: x.bits,
            flags: x
                .flags
                .iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<eid::IntegerIdBitFlag>, _>>()?,
        })
    }
}

//endregion

//region IntegerIdBitFlagYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct IntegerIdBitFlagYaml {
    identifier: String,

    #[serde(default)]
    documentation: String,

    #[serde(default)]
    bit: i8,

    #[serde(default)]
    bits: Vec<IntegerIdBitFlagBitYaml>,
}

impl convert::TryFrom<&IntegerIdBitFlagYaml> for eid::IntegerIdBitFlag {
    type Error = yaml::Error;

    fn try_from(x: &IntegerIdBitFlagYaml) -> Result<Self, Self::Error> {
        Ok(eid::IntegerIdBitFlag {
            identifier: x.identifier.to_owned(),
            documentation: x.documentation.to_owned(),
            bit: x.bit,
            bits: x
                .bits
                .iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<eid::IntegerIdBitFlagBit>, _>>()?,
        })
    }
}

impl convert::TryFrom<IntegerIdBitFlagYaml> for eid::IntegerIdBitFlag {
    type Error = yaml::Error;

    fn try_from(x: IntegerIdBitFlagYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//endregion

//region IntegerIdBitFlagBitYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct IntegerIdBitFlagBitYaml {
    index: i8,
    set: bool,
}

impl convert::TryFrom<&IntegerIdBitFlagBitYaml> for eid::IntegerIdBitFlagBit {
    type Error = yaml::Error;

    fn try_from(x: &IntegerIdBitFlagBitYaml) -> Result<Self, Self::Error> {
        Ok(eid::IntegerIdBitFlagBit {
            index: x.index,
            set: x.set,
        })
    }
}

impl convert::TryFrom<IntegerIdBitFlagBitYaml> for eid::IntegerIdBitFlagBit {
    type Error = yaml::Error;

    fn try_from(x: IntegerIdBitFlagBitYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//endregion
