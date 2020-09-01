//

use crate::{base::arity, mixin};

pub struct Ownable {
    pub owner_arity: mixin::MixinField<arity::ArityConstraint>,
}
