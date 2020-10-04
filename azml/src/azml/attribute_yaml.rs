//

use std::convert;

use crate::azml::{attribute, yaml};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AttributeYaml {
    identifier: String,

    kind: String,

    #[serde(default, rename = "final")]
    final_: bool,

    #[serde(default)]
    documentation: String,
}

impl convert::TryFrom<AttributeYaml> for attribute::Attribute {
    type Error = yaml::Error;

    fn try_from(x: AttributeYaml) -> Result<Self, Self::Error> {
        Ok(attribute::Attribute {
            identifier: x.identifier,
            kind: x.kind.into(),
            final_: x.final_,
            identifier_options: attribute::AttributeIdentifierOptions {},
            documentation: x.documentation,
        })
    }
}

impl convert::TryFrom<&AttributeYaml> for attribute::Attribute {
    type Error = yaml::Error;

    fn try_from(x: &AttributeYaml) -> Result<Self, Self::Error> {
        Ok(attribute::Attribute {
            identifier: x.identifier.to_owned(),
            kind: (&x.kind).into(),
            final_: x.final_,
            identifier_options: attribute::AttributeIdentifierOptions {},
            documentation: x.documentation.to_owned(),
        })
    }
}
