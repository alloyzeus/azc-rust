//

use std::convert;

use crate::azml::{adjunct::adjunct_value_object, yaml};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdjunctValueObjectYaml {}

impl convert::TryFrom<AdjunctValueObjectYaml> for adjunct_value_object::AdjunctValueObject {
    type Error = yaml::Error;

    fn try_from(_x: AdjunctValueObjectYaml) -> Result<Self, Self::Error> {
        Ok(adjunct_value_object::AdjunctValueObject {})
    }
}
