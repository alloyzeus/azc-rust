//

use std::convert;

use crate::azml::{primitive, value_object::value_object, yaml};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ValueObjectYaml {
    #[serde(default)]
    documentation: String,
}

impl convert::TryFrom<ValueObjectYaml> for value_object::ValueObject {
    type Error = yaml::Error;

    fn try_from(x: ValueObjectYaml) -> Result<Self, Self::Error> {
        Ok(value_object::ValueObject {
            documentation: x.documentation,
            definition: Box::new(value_object::ValueObjectPrimitive {
                documentation: "".to_owned(),
                type_name: primitive::PrimitiveType::Int8,
            }),
        })
    }
}
