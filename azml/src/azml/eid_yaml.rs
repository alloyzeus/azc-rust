//

use std::convert::{self, TryInto};

use crate::azml::{eid, yaml};

//region IntegerIdYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct IntegerIdYaml {
    #[serde(default)]
    total_bits: i8,

    significant_bits: i8,

    #[serde(default)]
    bitfield: Option<IntegerIdBitfieldYaml>,
}

impl convert::TryFrom<IntegerIdYaml> for eid::IntegerId {
    type Error = yaml::Error;

    fn try_from(x: IntegerIdYaml) -> Result<Self, Self::Error> {
        Ok(eid::IntegerId {
            total_bits: x.total_bits,
            significant_bits: x.significant_bits,
            bitfield: if let Some(y) = x.bitfield {
                y.try_into()?
            } else {
                eid::IntegerIdBitfield::default()
            },
        })
    }
}

//endregion

//region IntegerIdBitfield

#[derive(serde::Deserialize, serde::Serialize)]
struct IntegerIdBitfieldYaml {
    #[serde(default = "eid::IntegerIdBitfield::size_default")]
    pub size: i8,

    #[serde(default)]
    pub sub_fields: Vec<IntegerIdBitfieldSubFieldYaml>,

    #[serde(default)]
    pub inherits: Vec<IntegerIdBitfieldInheritYaml>,
}

impl convert::TryFrom<&IntegerIdBitfieldYaml> for eid::IntegerIdBitfield {
    type Error = yaml::Error;

    fn try_from(x: &IntegerIdBitfieldYaml) -> Result<Self, Self::Error> {
        Ok(eid::IntegerIdBitfield {
            size: x.size,
            sub_fields: x
                .sub_fields
                .iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<eid::IntegerIdBitfieldSubField>, _>>()?,
            inherits: x
                .inherits
                .iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<eid::IntegerIdBitfieldInherit>, _>>()?,
        })
    }
}

impl convert::TryFrom<IntegerIdBitfieldYaml> for eid::IntegerIdBitfield {
    type Error = yaml::Error;

    fn try_from(x: IntegerIdBitfieldYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//endregion

//region IntegerIdBitfieldSubField

#[derive(serde::Deserialize, serde::Serialize)]
struct IntegerIdBitfieldSubFieldYaml {
    pub identifier: String,
    pub documentation: String,
    pub bits: Vec<IntegerIdBitfieldSubFieldBitYaml>,
}

impl convert::TryFrom<&IntegerIdBitfieldSubFieldYaml> for eid::IntegerIdBitfieldSubField {
    type Error = yaml::Error;

    fn try_from(x: &IntegerIdBitfieldSubFieldYaml) -> Result<Self, Self::Error> {
        Ok(eid::IntegerIdBitfieldSubField {
            identifier: x.identifier.to_owned(),
            documentation: x.documentation.to_owned(),
            bits: x
                .bits
                .iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<eid::IntegerIdBitfieldSubFieldBit>, _>>()?,
        })
    }
}

impl convert::TryFrom<IntegerIdBitfieldSubFieldYaml> for eid::IntegerIdBitfieldSubField {
    type Error = yaml::Error;

    fn try_from(x: IntegerIdBitfieldSubFieldYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//endregion

//region IntegerIdBitfieldSubFieldBit

#[derive(serde::Deserialize, serde::Serialize)]
struct IntegerIdBitfieldSubFieldBitYaml {
    index: i8,
    set: bool,
}

impl convert::TryFrom<&IntegerIdBitfieldSubFieldBitYaml> for eid::IntegerIdBitfieldSubFieldBit {
    type Error = yaml::Error;

    fn try_from(x: &IntegerIdBitfieldSubFieldBitYaml) -> Result<Self, Self::Error> {
        Ok(eid::IntegerIdBitfieldSubFieldBit {
            index: x.index,
            set: x.set,
        })
    }
}

impl convert::TryFrom<IntegerIdBitfieldSubFieldBitYaml> for eid::IntegerIdBitfieldSubFieldBit {
    type Error = yaml::Error;

    fn try_from(x: IntegerIdBitfieldSubFieldBitYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//endregion

//region IntegerIdBitfieldInheritYaml

#[derive(serde::Deserialize, serde::Serialize)]
struct IntegerIdBitfieldInheritYaml {
    host: i8,
    size: i8,
}

impl convert::TryFrom<&IntegerIdBitfieldInheritYaml> for eid::IntegerIdBitfieldInherit {
    type Error = yaml::Error;

    fn try_from(x: &IntegerIdBitfieldInheritYaml) -> Result<Self, Self::Error> {
        Ok(eid::IntegerIdBitfieldInherit {
            host: x.host,
            size: x.size,
        })
    }
}

impl convert::TryFrom<IntegerIdBitfieldInheritYaml> for eid::IntegerIdBitfieldInherit {
    type Error = yaml::Error;

    fn try_from(x: IntegerIdBitfieldInheritYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//endregion
