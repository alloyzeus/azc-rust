//

use crate::azml::{authorization, mixin};

// Special mixin.
//
// Creation is a special mixin which defines the rule for the creation
// of any instance.
#[derive(Clone, Debug)]
pub struct Creation {
    pub documentation: String,
    pub authorization: authorization::Authorization,
}

impl mixin::MixinDefinition for Creation {}
