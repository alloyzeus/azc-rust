//

use crate::azml::cardinality;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CardinalityConstraintYaml {
    min: i64,
    max: i64,
}

impl Default for CardinalityConstraintYaml {
    fn default() -> CardinalityConstraintYaml {
        CardinalityConstraintYaml { min: -1, max: -1 }
    }
}

impl From<&CardinalityConstraintYaml> for cardinality::CardinalityConstraint {
    fn from(x: &CardinalityConstraintYaml) -> cardinality::CardinalityConstraint {
        cardinality::CardinalityConstraint {
            min: x.min,
            max: x.max,
        }
    }
}

impl From<CardinalityConstraintYaml> for cardinality::CardinalityConstraint {
    fn from(x: CardinalityConstraintYaml) -> cardinality::CardinalityConstraint {
        (&x).into()
    }
}
