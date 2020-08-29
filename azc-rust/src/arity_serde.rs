//

use crate::arity;
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

impl Into<arity::ArityConstraint> for ArityConstraintSerde {
    fn into(self) -> arity::ArityConstraint {
        arity::ArityConstraint {
            min: self.min,
            max: self.max,
        }
    }
}
