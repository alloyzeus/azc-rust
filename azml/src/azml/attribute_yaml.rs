//

use std::convert::{self, TryInto};

use crate::azml::yaml;

use super::attribute;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AttributeYaml {
    name: String,

    kind: String,

    #[serde(default, rename = "final")]
    final_: bool,

    #[serde(default)]
    documentation: String,
}

impl convert::TryFrom<AttributeYaml> for attribute::Attribute {
    type Error = yaml::Error;

    fn try_from(x: AttributeYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

impl convert::TryFrom<&AttributeYaml> for attribute::Attribute {
    type Error = yaml::Error;

    fn try_from(x: &AttributeYaml) -> Result<Self, Self::Error> {
        Ok(attribute::Attribute {
            name: x.name.to_owned(),
            kind: (&x.kind).into(),
            final_: x.final_,
            name_options: attribute::AttributeNameOptions {},
            documentation: x.documentation.to_owned(),
        })
    }
}
