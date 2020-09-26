//

#[derive(Clone, Debug)]
pub struct ArityConstraint {
    pub min: i64,
    pub max: i64,
}

impl Default for ArityConstraint {
    fn default() -> ArityConstraint {
        ArityConstraint { min: -1, max: -1 }
    }
}
