//

use crate::{base::arity, mixin};

pub struct Ownership {
    pub owner_arity: mixin::MixinField<arity::ArityConstraint>,
}
