//

//TODO:
// - transferability: transferable (adjunct entity) or final (adjunct entity and value_object)

use crate::azml::{cardinality, mixin};

#[derive(Clone, Debug)]
pub struct Ownership {
    // Cardinality of owned -> owner
    pub owner_cardinality: mixin::MixinField<cardinality::CardinalityConstraint>,
    // Cardinality of owned <- owner
    pub owned_cardinality: mixin::MixinField<cardinality::CardinalityConstraint>,
}

impl mixin::MixinDefinition for Ownership {}
