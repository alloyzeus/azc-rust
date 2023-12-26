//

use std::convert::{self, TryInto};

use crate::azml::{symbol::{self, SymbolRef}, value_object::value_object, yaml};

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
                Ok(Self {
                    definition: Box::new(value_object::ValueObjectStruct::from(def.try_into()?)),
                })
            }
            "alias" => {
                let def: ValueObjectAliasYaml = yaml::from_value(x.parameters)?;
                let type_ref = SymbolRef::from(def.data_type);
                Ok(Self {
                    definition: Box::new(value_object::ValueObjectAlias {
                        data_type: type_ref,
                    }),
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
        let type_ref = SymbolRef::from(x.data_type);
        Ok(Self { data_type: type_ref })
    }
}

//----

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ValueObjectStructYaml {
    #[serde(default)]
    key: Option<ValueObjectStructKeyYaml>,

    #[serde(default)]
    fields: Vec<ValueObjectStructFieldYaml>,
}

impl convert::TryFrom<ValueObjectStructYaml> for value_object::ValueObjectStruct {
    type Error = yaml::Error;

    fn try_from(x: ValueObjectStructYaml) -> Result<Self, Self::Error> {
        Ok(Self {
            key: if let Some(k) = x.key {
                Some(value_object::ValueObjectStructKey::try_from(k)?)
            } else {
                None
            },
            fields: x
                .fields
                .into_iter()
                .map(|o| value_object::ValueObjectStructField::try_from(o))
                .collect::<Result<Vec<value_object::ValueObjectStructField>, _>>()?,
        })
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ValueObjectStructKeyYaml {
    fields: Vec<String>,
}

impl convert::TryFrom<ValueObjectStructKeyYaml> for value_object::ValueObjectStructKey {
    type Error = yaml::Error;

    fn try_from(x: ValueObjectStructKeyYaml) -> Result<Self, Self::Error> {
        Ok(Self {
            fields: x.fields.clone(),
        })
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ValueObjectStructFieldYaml {
    identifier: String,
    data_type: String, // SymbolRef
                       //TODO: storage directives, visibility, etc.
}

impl convert::TryFrom<ValueObjectStructFieldYaml> for value_object::ValueObjectStructField {
    type Error = yaml::Error;

    fn try_from(x: ValueObjectStructFieldYaml) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: x.identifier,
            data_type: symbol::SymbolRef::from(x.data_type),
        })
    }
}
