//

use crate::azml::{arity, mixin};

#[derive(Clone, Debug)]
pub struct Ownership {
    pub owner_arity: mixin::MixinField<arity::ArityConstraint>,
    //TODO: transferability: transferable (adjunct entity) or final (adjunct entity and value-object)
}

impl mixin::MixinDefinition for Ownership {}
