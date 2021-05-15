//

use super::{creation, deletion, expiration};

#[derive(Clone, Debug)]
pub struct Lifecycle {
    pub creation: creation::Creation,
    pub deletion: deletion::Deletion,
    pub expiration: expiration::Expiration,
}
