//

use super::{creation::creation, deletion::deletion};

#[derive(Clone, Debug)]
pub struct Lifecycle {
    pub creation: creation::Creation,
    pub deletion: deletion::Deletion,
}
