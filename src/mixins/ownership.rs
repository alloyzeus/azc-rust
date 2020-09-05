//

use crate::{base::arity, mixin};

#[derive(Debug)]
pub struct Ownership {
    pub owner_arity: mixin::MixinField<arity::ArityConstraint>,
}

impl mixin::MixinDefinition for Ownership {}
