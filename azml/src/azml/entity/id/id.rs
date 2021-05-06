//

use super::{id_num, ref_key};

// Id describes primary identifier of an entity type.

#[derive(Clone, Debug)]
pub struct Id {
    pub num: id_num::IdNum,
    pub ref_key: ref_key::RefKey,
}
