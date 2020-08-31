//

use crate::base::arity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ArityConstraintSerde {
    min: i64,
    max: i64,
}

impl Default for ArityConstraintSerde {
    fn default() -> ArityConstraintSerde {
        ArityConstraintSerde { min: -1, max: -1 }
    }
}

impl From<ArityConstraintSerde> for arity::ArityConstraint {
    fn from(x: ArityConstraintSerde) -> arity::ArityConstraint {
        arity::ArityConstraint {
            min: x.min,
            max: x.max,
        }
    }
}
