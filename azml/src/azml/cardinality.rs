//

#[derive(Clone, Debug)]
pub struct CardinalityConstraint {
    pub min: i64,
    pub max: i64,
}

impl Default for CardinalityConstraint {
    fn default() -> Self {
        Self { min: -1, max: -1 }
    }
}
