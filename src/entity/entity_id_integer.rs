//

use crate::entity::entity_id;

#[derive(Debug)]
pub struct EntityIdInteger {
    // The number of bits. Note that the actual types used are rounded up
    // to the power of 2. to ensure compatibility, we use signed integers,
    // so the actual space is limited to up to 63.
    // Value from 1 to 7 will usually use 8bit signed integer, 8 to 15 use
    // 16bit signed integer, 16 to 31 use 32bit signed integer, 32 to 63 use
    // 64bit signed integer.
    pub space: i8,
}

impl entity_id::EntityIdParameters for EntityIdInteger {}
