//

use crate::{arity_serde, mixins::ownership};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OwnableSerde {
    owner_arity: arity_serde::ArityConstraintSerde,
}

impl From<OwnableSerde> for ownership::Ownable {
    fn from(x: OwnableSerde) -> ownership::Ownable {
        ownership::Ownable {
            owner_arity: x.owner_arity.into(),
        }
    }
}
