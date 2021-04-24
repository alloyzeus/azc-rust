//

use std::convert::{self, TryInto};

use crate::azml::{id_num, yaml};

//region IntegerIdNumYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct IntegerIdNumYaml {
    #[serde(default)]
    total_bits: i8,

    identifier_bits: i8,

    #[serde(default)]
    bitfield: Option<IntegerIdNumBitfieldYaml>,
}

impl convert::TryFrom<IntegerIdNumYaml> for id_num::IntegerIdNum {
    type Error = yaml::Error;

    fn try_from(x: IntegerIdNumYaml) -> Result<Self, Self::Error> {
        Ok(id_num::IntegerIdNum {
            total_bits: x.total_bits,
            identifier_bits: x.identifier_bits,
            bitfield: if let Some(y) = x.bitfield {
                y.try_into()?
            } else {
                id_num::IntegerIdNumBitfield::default()
            },
        })
    }
}

//endregion

//region IntegerIdNumBitfield

#[derive(serde::Deserialize, serde::Serialize)]
struct IntegerIdNumBitfieldYaml {
    #[serde(default = "id_num::IntegerIdNumBitfield::size_default")]
    pub size: i8,

    #[serde(default)]
    pub sub_fields: Vec<IntegerIdNumBitfieldSubFieldYaml>,

    #[serde(default)]
    pub inherits: Vec<IntegerIdNumBitfieldInheritYaml>,
}

impl convert::TryFrom<&IntegerIdNumBitfieldYaml> for id_num::IntegerIdNumBitfield {
    type Error = yaml::Error;

    fn try_from(x: &IntegerIdNumBitfieldYaml) -> Result<Self, Self::Error> {
        Ok(id_num::IntegerIdNumBitfield {
            size: x.size,
            sub_fields: x
                .sub_fields
                .iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<id_num::IntegerIdNumBitfieldSubField>, _>>()?,
            inherits: x
                .inherits
                .iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<id_num::IntegerIdNumBitfieldInherit>, _>>()?,
        })
    }
}

impl convert::TryFrom<IntegerIdNumBitfieldYaml> for id_num::IntegerIdNumBitfield {
    type Error = yaml::Error;

    fn try_from(x: IntegerIdNumBitfieldYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//endregion

//region IntegerIdNumBitfieldSubField

#[derive(serde::Deserialize, serde::Serialize)]
struct IntegerIdNumBitfieldSubFieldYaml {
    pub identifier: String,
    pub documentation: String,
    pub size: i8,

    #[serde(default)]
    pub values: Vec<IntegerIdNumBitfieldSubFieldValueYaml>,
}

impl convert::TryFrom<&IntegerIdNumBitfieldSubFieldYaml> for id_num::IntegerIdNumBitfieldSubField {
    type Error = yaml::Error;

    fn try_from(x: &IntegerIdNumBitfieldSubFieldYaml) -> Result<Self, Self::Error> {
        Ok(id_num::IntegerIdNumBitfieldSubField {
            identifier: x.identifier.to_owned(),
            documentation: x.documentation.to_owned(),
            size: x.size,
            values: x
                .values
                .iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<id_num::IntegerIdNumBitfieldSubFieldValue>, _>>()?,
        })
    }
}

impl convert::TryFrom<IntegerIdNumBitfieldSubFieldYaml> for id_num::IntegerIdNumBitfieldSubField {
    type Error = yaml::Error;

    fn try_from(x: IntegerIdNumBitfieldSubFieldYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//endregion

//region IntegerIdNumBitfieldSubFieldValue

#[derive(serde::Deserialize, serde::Serialize)]
struct IntegerIdNumBitfieldSubFieldValueYaml {
    identifier: String,

    #[serde(default)]
    documentation: String,

    #[serde(default)]
    sub_fields: Vec<IntegerIdNumBitfieldSubFieldYaml>,
}

impl convert::TryFrom<&IntegerIdNumBitfieldSubFieldValueYaml>
    for id_num::IntegerIdNumBitfieldSubFieldValue
{
    type Error = yaml::Error;

    fn try_from(x: &IntegerIdNumBitfieldSubFieldValueYaml) -> Result<Self, Self::Error> {
        Ok(id_num::IntegerIdNumBitfieldSubFieldValue {
            identifier: x.identifier.to_owned(),
            documentation: x.documentation.to_owned(),
            sub_fields: x
                .sub_fields
                .iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<id_num::IntegerIdNumBitfieldSubField>, _>>()?,
        })
    }
}

impl convert::TryFrom<IntegerIdNumBitfieldSubFieldValueYaml>
    for id_num::IntegerIdNumBitfieldSubFieldValue
{
    type Error = yaml::Error;

    fn try_from(x: IntegerIdNumBitfieldSubFieldValueYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//endregion

//region IntegerIdNumBitfieldSubFieldBit

#[derive(serde::Deserialize, serde::Serialize)]
struct IntegerIdNumBitfieldSubFieldBitYaml {
    index: i8,
    set: bool,
}

impl convert::TryFrom<&IntegerIdNumBitfieldSubFieldBitYaml>
    for id_num::IntegerIdNumBitfieldSubFieldBit
{
    type Error = yaml::Error;

    fn try_from(x: &IntegerIdNumBitfieldSubFieldBitYaml) -> Result<Self, Self::Error> {
        Ok(id_num::IntegerIdNumBitfieldSubFieldBit {
            index: x.index,
            set: x.set,
        })
    }
}

impl convert::TryFrom<IntegerIdNumBitfieldSubFieldBitYaml>
    for id_num::IntegerIdNumBitfieldSubFieldBit
{
    type Error = yaml::Error;

    fn try_from(x: IntegerIdNumBitfieldSubFieldBitYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//endregion

//region IntegerIdNumBitfieldInheritYaml

#[derive(serde::Deserialize, serde::Serialize)]
struct IntegerIdNumBitfieldInheritYaml {
    host: i8,
    size: i8,
}

impl convert::TryFrom<&IntegerIdNumBitfieldInheritYaml> for id_num::IntegerIdNumBitfieldInherit {
    type Error = yaml::Error;

    fn try_from(x: &IntegerIdNumBitfieldInheritYaml) -> Result<Self, Self::Error> {
        Ok(id_num::IntegerIdNumBitfieldInherit {
            host: x.host,
            size: x.size,
        })
    }
}

impl convert::TryFrom<IntegerIdNumBitfieldInheritYaml> for id_num::IntegerIdNumBitfieldInherit {
    type Error = yaml::Error;

    fn try_from(x: IntegerIdNumBitfieldInheritYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//endregion
