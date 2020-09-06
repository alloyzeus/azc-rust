//

use crate::azml::arity;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ArityConstraintYaml {
    min: i64,
    max: i64,
}

impl Default for ArityConstraintYaml {
    fn default() -> ArityConstraintYaml {
        ArityConstraintYaml { min: -1, max: -1 }
    }
}

impl From<ArityConstraintYaml> for arity::ArityConstraint {
    fn from(x: ArityConstraintYaml) -> arity::ArityConstraint {
        arity::ArityConstraint {
            min: x.min,
            max: x.max,
        }
    }
}
