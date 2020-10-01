//

use std::convert;

use crate::azml::{data_type, value_object::value_object, yaml};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ValueObjectYaml {
    data_type: String,
}

impl convert::TryFrom<ValueObjectYaml> for value_object::ValueObject {
    type Error = yaml::Error;

    fn try_from(x: ValueObjectYaml) -> Result<Self, Self::Error> {
        let dtype = x.data_type.parse::<data_type::DataType>();
        match dtype {
            Ok(e) => {
                match e {
                    data_type::DataType::Struct => Ok(value_object::ValueObject {
                        data_type: e,
                        struct_: Some(value_object::ValueObjectStruct {
                            documentation: "".to_owned(),
                        }), //TODO: fill this
                    }),
                    _ => Ok(value_object::ValueObject {
                        data_type: e,
                        struct_: None,
                    }),
                }
            }
            Err(e) => Err(Self::Error::Msg(e.to_owned())),
        }
    }
}
