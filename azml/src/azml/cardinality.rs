//

#[derive(Clone, Debug)]
pub struct CardinalityConstraint {
    pub min: i64,
    pub max: i64,
}

impl CardinalityConstraint {
    pub fn unconstrained(&self) -> bool {
        self.min == -1 && self.max == -1
    }

    pub fn at_max_one(&self) -> bool {
        (self.min == -1 || self.min == 0) && self.max == 1
    }
}

impl Default for CardinalityConstraint {
    fn default() -> Self {
        Self { min: -1, max: -1 }
    }
}
