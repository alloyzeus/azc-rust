//

use crate::azml::mixin;

// Special mixin.
//
// Creation is a special mixin which defines the rule for the creation
// of any instance.
#[derive(Clone, Debug)]
pub struct Creation {
    pub documentation: String,
    pub allow_cross_process_callers: bool,
}

impl mixin::MixinDefinition for Creation {}
