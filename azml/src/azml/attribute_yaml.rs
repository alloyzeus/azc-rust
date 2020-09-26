//

use std::convert;

use crate::azml::{attribute, yaml};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AttributeYaml {
    kind: String,

    #[serde(default)]
    final_: bool,
}

impl convert::TryFrom<AttributeYaml> for attribute::Attribute {
    type Error = yaml::Error;

    fn try_from(x: AttributeYaml) -> Result<Self, Self::Error> {
        Ok(attribute::Attribute {
            kind: x.kind,
            final_: x.final_,
        })
    }
}

impl convert::TryFrom<&AttributeYaml> for attribute::Attribute {
    type Error = yaml::Error;

    fn try_from(x: &AttributeYaml) -> Result<Self, Self::Error> {
        Ok(attribute::Attribute {
            kind: x.kind.to_owned(),
            final_: x.final_,
        })
    }
}
