//

//TODO:
// - transferability: transferable (adjunct entity) or final (adjunct entity and value_object)

use crate::azml::{arity, mixin};

#[derive(Clone, Debug)]
pub struct Ownership {
    pub owner_arity: mixin::MixinField<arity::ArityConstraint>,
}

impl mixin::MixinDefinition for Ownership {}
