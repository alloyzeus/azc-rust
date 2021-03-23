//

use std::{convert, convert::TryInto};

use crate::azfl::ownership::ownership;
use crate::azml::{arity, arity_yaml, error, mixin, mixin_yaml};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct OwnershipYaml {
    owner_arity: mixin_yaml::MixinFieldYaml<arity_yaml::ArityConstraintYaml>,
}

impl convert::TryFrom<OwnershipYaml> for ownership::Ownership {
    type Error = error::Error;

    fn try_from(x: OwnershipYaml) -> Result<Self, Self::Error> {
        //TODO: use generic TryFrom
        let owner_arity = mixin::MixinField::<arity::ArityConstraint> {
            overridable: x.owner_arity.overridable,
            value: x.owner_arity.value.try_into()?,
        };
        Ok(ownership::Ownership {
            owner_arity: owner_arity,
        })
    }
}