//

use std::convert::{self, TryInto};

use crate::azml::{data_type, value_object::value_object, yaml};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ValueObjectYaml {
    kind: String,
    parameters: yaml::Value,
}

impl convert::TryFrom<ValueObjectYaml> for value_object::ValueObject {
    type Error = yaml::Error;

    fn try_from(x: ValueObjectYaml) -> Result<Self, Self::Error> {
        match x.kind.as_ref() {
            "struct" => {
                let def: ValueObjectStructYaml = yaml::from_value(x.parameters)?;
                Ok(value_object::ValueObject {
                    definition: Box::new(value_object::ValueObjectStruct::from(def.try_into()?)),
                })
            }
            "alias" => {
                let def: ValueObjectAliasYaml = yaml::from_value(x.parameters)?;
                let dtype = def.data_type.parse::<data_type::DataType>()?;
                Ok(value_object::ValueObject {
                    definition: Box::new(value_object::ValueObjectAlias { data_type: dtype }),
                })
            }
            _ => Err(yaml::Error::Msg("Unrecogized value object kind".to_owned())),
        }
    }
}

//----

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ValueObjectAliasYaml {
    data_type: String,
}

impl convert::TryFrom<ValueObjectAliasYaml> for value_object::ValueObjectAlias {
    type Error = yaml::Error;

    fn try_from(x: ValueObjectAliasYaml) -> Result<Self, Self::Error> {
        let dtype = x.data_type.parse::<data_type::DataType>()?;
        Ok(value_object::ValueObjectAlias { data_type: dtype })
    }
}

//----

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ValueObjectStructYaml {}

impl convert::TryFrom<ValueObjectStructYaml> for value_object::ValueObjectStruct {
    type Error = yaml::Error;

    fn try_from(_x: ValueObjectStructYaml) -> Result<Self, Self::Error> {
        Ok(value_object::ValueObjectStruct {})
    }
}
