//

use crate::azml::entity::entity_id;

#[derive(Clone, Debug)]
pub struct EntityIdInteger {
    // The number of bits. Note that the actual types used are rounded up
    // to the power of 2. to ensure compatibility, we use signed integers,
    // so the actual space is limited to up to 63.
    //
    // For the implementations, value from 1 to 15 will usually use 16bit
    // signed integer type (e.g., int16, smallint), 16 to 31 use
    // 32bit signed integer (e.g., int32, integer), 32 to 63 use
    // 64bit signed integer (e.g., int64, bigint). Note that
    // we use 16bit integer as the smallest to ensure compatibility and
    // consistency.
    //
    // Negative values are unused. Zero, as the default value, should not be
    // used as the identifier of any valid entity instance*.
    //
    // * For example, 0 in the context of user ID could be used to indicate
    //   nobody.
    pub bits: i8,
    //TODO: additional attributes (flags) encoding. these attributes are
    // part of the identity for the entity's lifetime. For example,
    // there's two types of application: internal and third-party. this
    // information could be included in the ID as bit flag.
    // the number of attributes could be included is strictly limited.
}

impl entity_id::EntityIdDefinition for EntityIdInteger {}

pub trait EntityIdIntegerEncoding: mopa::Any + std::fmt::Debug {}

mopafy!(EntityIdIntegerEncoding);
