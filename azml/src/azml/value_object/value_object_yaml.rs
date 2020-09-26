//

use std::convert;

use crate::azml::{data_type, value_object::value_object, yaml};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ValueObjectYaml {
    #[serde(default)]
    documentation: String,

    data_type: String,
}

impl convert::TryFrom<ValueObjectYaml> for value_object::ValueObject {
    type Error = yaml::Error;

    fn try_from(x: ValueObjectYaml) -> Result<Self, Self::Error> {
        let dtype = x.data_type.parse::<data_type::DataType>();
        match dtype {
            Ok(e) => Ok(value_object::ValueObject {
                documentation: x.documentation,
                definition: Box::new(value_object::ValueObjectPrimitive {
                    documentation: "".to_owned(),
                    data_type: e,
                }),
            }),
            Err(e) => Err(Self::Error::Msg(e.to_owned())),
        }
    }
}
