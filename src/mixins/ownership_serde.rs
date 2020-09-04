//

use serde::{Deserialize, Serialize};
use std::{convert, convert::TryInto};

use crate::{base::arity, base::arity_serde, base::error, mixin, mixin_serde, mixins::ownership};

#[derive(Serialize, Deserialize)]
pub struct OwnershipSerde {
    owner_arity: mixin_serde::MixinFieldSerde<arity_serde::ArityConstraintSerde>,
}

impl convert::TryFrom<OwnershipSerde> for ownership::Ownership {
    type Error = error::Error;

    fn try_from(x: OwnershipSerde) -> Result<Self, Self::Error> {
        //TODO: use generic TryFrom
        let owner_arity = mixin::MixinField::<arity::ArityConstraint> {
            overridable: x.owner_arity.overridable,
            value: x.owner_arity.value.try_into().unwrap(), //TODO: use `?`
        };
        Ok(ownership::Ownership {
            owner_arity: owner_arity,
        })
    }
}
