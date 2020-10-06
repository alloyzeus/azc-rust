//

use std::convert::{self, TryInto};

use crate::azml::{oid, yaml};

//region IntegerIdYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct IntegerIdYaml {
    bits: i8,

    #[serde(default)]
    flags: Vec<IntegerIdBitFlagYaml>,
}

impl convert::TryFrom<IntegerIdYaml> for oid::IntegerId {
    type Error = yaml::Error;

    fn try_from(x: IntegerIdYaml) -> Result<Self, Self::Error> {
        Ok(oid::IntegerId {
            bits: x.bits,
            flags: x
                .flags
                .iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<oid::IntegerIdBitFlag>, _>>()?,
        })
    }
}

//endregion

//region IntegerIdBitFlagYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct IntegerIdBitFlagYaml {
    identifier: String,
    bit: i8,
}

impl convert::TryFrom<IntegerIdBitFlagYaml> for oid::IntegerIdBitFlag {
    type Error = yaml::Error;

    fn try_from(x: IntegerIdBitFlagYaml) -> Result<Self, Self::Error> {
        Ok(oid::IntegerIdBitFlag {
            identifier: x.identifier,
            bit: x.bit,
        })
    }
}

impl convert::TryFrom<&IntegerIdBitFlagYaml> for oid::IntegerIdBitFlag {
    type Error = yaml::Error;

    fn try_from(x: &IntegerIdBitFlagYaml) -> Result<Self, Self::Error> {
        Ok(oid::IntegerIdBitFlag {
            identifier: x.identifier.to_owned(),
            bit: x.bit,
        })
    }
}

//endregion
