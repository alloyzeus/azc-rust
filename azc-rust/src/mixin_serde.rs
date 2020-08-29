//

use crate::mixin;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MixinSerde {
    name: String,
}

impl Into<mixin::Mixin> for MixinSerde {
    fn into(self) -> mixin::Mixin {
        mixin::Mixin { name: self.name }
    }
}
