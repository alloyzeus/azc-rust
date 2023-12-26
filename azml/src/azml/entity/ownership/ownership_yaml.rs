//

use std::{convert, convert::TryInto};

use crate::azml::{cardinality, cardinality_yaml, error, mixin, mixin_yaml};

use super::ownership;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct OwnershipYaml {
    owner_cardinality: mixin_yaml::MixinFieldYaml<cardinality_yaml::CardinalityConstraintYaml>,
    owned_cardinality: mixin_yaml::MixinFieldYaml<cardinality_yaml::CardinalityConstraintYaml>,
}

impl convert::TryFrom<OwnershipYaml> for ownership::Ownership {
    type Error = error::Error;

    fn try_from(x: OwnershipYaml) -> Result<Self, Self::Error> {
        //TODO: use generic TryFrom
        let owner_cardinality = mixin::MixinField::<cardinality::CardinalityConstraint> {
            overridable: x.owner_cardinality.overridable,
            value: x.owner_cardinality.value.try_into()?,
        };
        let owned_cardinality = mixin::MixinField::<cardinality::CardinalityConstraint> {
            overridable: x.owned_cardinality.overridable,
            value: x.owned_cardinality.value.try_into()?,
        };
        Ok(Self {
            owner_cardinality,
            owned_cardinality,
        })
    }
}
