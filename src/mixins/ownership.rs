//

use crate::base::arity;

pub struct Ownable {
    pub owner_arity: arity::ArityConstraint,
}
