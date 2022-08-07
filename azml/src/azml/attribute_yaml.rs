//

use std::convert::{self, TryInto};

use super::{attribute, yaml};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AttributeYaml {
    name: String,

    kind: String,

    finality: String,

    #[serde(default)]
    name_options: AttributeNameOptionsYaml,

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
        Ok(Self {
            name: x.name.to_owned(),
            kind: (&x.kind).into(),
            finality: (&x.finality).try_into()?,
            name_options: (&x.name_options).try_into()?,
            documentation: x.documentation.to_owned(),
        })
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AttributeNameOptionsYaml {
    #[serde(default)]
    snake_case: String,
}

impl Default for AttributeNameOptionsYaml {
    fn default() -> Self {
        Self {
            snake_case: "".to_owned(),
        }
    }
}

impl convert::TryFrom<AttributeNameOptionsYaml> for attribute::AttributeNameOptions {
    type Error = yaml::Error;

    fn try_from(x: AttributeNameOptionsYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

impl convert::TryFrom<&AttributeNameOptionsYaml> for attribute::AttributeNameOptions {
    type Error = yaml::Error;

    fn try_from(x: &AttributeNameOptionsYaml) -> Result<Self, Self::Error> {
        Ok(Self {
            snake_case: x.snake_case.to_owned(),
        })
    }
}
