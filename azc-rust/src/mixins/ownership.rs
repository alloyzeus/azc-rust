//

use crate::arity;

pub struct Ownable {
    pub owner_arity: arity::ArityConstraint,
}
