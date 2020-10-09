//

//region IntegerId

#[derive(Clone, Debug)]
pub struct IntegerId {
    pub total_bits: i8,

    // The number of bits. Note that the actual types used are rounded up
    // to the power of 2. to ensure compatibility, we use signed 64bit integers,
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
    pub significant_bits: i8,

    pub bitfield: IntegerIdBitfield,

    //TODO: additional attributes (flags) encoding. these attributes are
    // part of the identity for the entity's lifetime. For example,
    // there's two types of application: internal and third-party. this
    // information could be included in the ID as bit flag.
    // the number of attributes could be included is strictly limited.
    pub flags: Vec<IntegerIdBitFlag>,
    //TODO: methods definition. e.g., IsConfidentialUserAgent requires both
    // bit 62 and 61 to be set, and IsService requires bit 62 to be unset and
    // bit 61 to be set.
    // Probably use a different approach, e.g.,
    // - 0b??1000 => PublicUserAgent
    // - 0b??1100 => ConfidentialUserAgent
    // - 0b??0100 => Service
}

impl IntegerId {
    pub fn primitive_size(&self) -> i8 {
        if self.total_bits > 0 {
            self.total_bits
        } else {
            match self.significant_bits {
                d if d < 16 => 16,
                d if d < 32 => 32,
                d if d < 64 => 64,
                _ => panic!(
                    "Unsupported bits value {} (bits value must be smaller than 64)",
                    self.significant_bits
                ),
            }
        }
    }
}

//endregion

//region IntegerIdBitFlag

#[derive(Clone, Debug)]
pub struct IntegerIdBitFlag {
    pub bit: i8,
    pub identifier: String,
    pub documentation: String,
    pub bits: Vec<IntegerIdBitFlagBit>,
}

#[derive(Clone, Debug)]
pub struct IntegerIdBitFlagBit {
    pub index: i8,
    pub set: bool,
}

//endregion

//region IntegerIdBitfield

#[derive(Clone, Debug)]
pub struct IntegerIdBitfield {
    pub size: i8,
    pub sub_fields: Vec<IntegerIdBitfieldSubField>,
}

impl Default for IntegerIdBitfield {
    fn default() -> IntegerIdBitfield {
        IntegerIdBitfield {
            size: -1,
            sub_fields: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IntegerIdBitfieldSubField {
    pub identifier: String,
    pub documentation: String,
    pub bits: Vec<IntegerIdBitfieldSubFieldBit>,
}

#[derive(Clone, Debug)]
pub struct IntegerIdBitfieldSubFieldBit {
    pub index: i8,
    pub set: bool,
}

//endregion
