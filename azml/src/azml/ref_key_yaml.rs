//

use std::convert::{self, TryInto};

use crate::azml::{ref_key, yaml};

//region RefKeyYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RefKeyYaml {
    #[serde(default)]
    pub included_attributes: Vec<RefKeyIncludedAttributeYaml>,
}

impl convert::TryFrom<RefKeyYaml> for ref_key::RefKey {
    type Error = yaml::Error;
    fn try_from(x: RefKeyYaml) -> Result<Self, Self::Error> {
        Ok(ref_key::RefKey {
            included_attributes: x
                .included_attributes
                .iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<ref_key::RefKeyIncludedAttribute>, _>>()?,
        })
    }
}

impl convert::TryFrom<&RefKeyYaml> for ref_key::RefKey {
    type Error = yaml::Error;
    fn try_from(x: &RefKeyYaml) -> Result<Self, Self::Error> {
        Ok(ref_key::RefKey {
            included_attributes: x
                .included_attributes
                .iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<ref_key::RefKeyIncludedAttribute>, _>>()?,
        })
    }
}

//endregion

//region RefKeyIncludedAttributeYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RefKeyIncludedAttributeYaml {
    pub name: String,
}

impl convert::TryFrom<RefKeyIncludedAttributeYaml> for ref_key::RefKeyIncludedAttribute {
    type Error = yaml::Error;
    fn try_from(x: RefKeyIncludedAttributeYaml) -> Result<Self, Self::Error> {
        Ok(ref_key::RefKeyIncludedAttribute { name: x.name })
    }
}

impl convert::TryFrom<&RefKeyIncludedAttributeYaml> for ref_key::RefKeyIncludedAttribute {
    type Error = yaml::Error;
    fn try_from(x: &RefKeyIncludedAttributeYaml) -> Result<Self, Self::Error> {
        Ok(ref_key::RefKeyIncludedAttribute {
            name: x.name.to_owned(),
        })
    }
}

//endregion
