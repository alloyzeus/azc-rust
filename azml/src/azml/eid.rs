//

//region IntegerId

#[derive(Clone, Debug)]
pub struct IntegerId {
    // This field is used to ensure that the IDs fit in the stated value;
    // the compiler will return error if the IDs doesn't fit in
    // total_bits.
    //
    // Valid value is any of 16 (analogous to int16_t in C/C++,
    // smallint in SQL), 32 (analogous to int32_t in C/C++,
    // int in SQL) and 64 (analogous to int64_t in C/C++, bigint in SQL).
    //
    // If not provided, total_bits will be determined by picking the
    // option value a step larger than the value provided to
    // significant_bits.
    pub total_bits: i8,

    // The number of bits. Note that the actual types used are rounded up
    // to the next power of 2. to ensure compatibility, we use signed 64bit
    // integers, so the maximum usable bits is limited to 63.
    //
    // For the implementations, value from 1 to 15 will usually use 16bit
    // signed integer type (e.g., int16, smallint), 16 to 31 use
    // 32bit signed integer (e.g., int32, integer), 32 to 63 use
    // 64bit signed integer (e.g., int64, bigint). Note that
    // we use 16bit integer as the smallest to ensure compatibility and
    // consistency.
    //
    // Negative values are unused. Zero, as the default value, should not be
    // used as the identifier of any valid entity instance -- for example,
    // 0 in the context of user ID could be used to indicate nobody.
    pub significant_bits: i8,

    // Additional attributes (flags) encoding. these attributes are
    // part of the identity for the entity's lifetime.
    //
    // As an example use case, there's two types of application, which is
    // first-part and third-party. First-party applications could access
    // more API than that of available to third parties. These API will need
    // to check whether the application which made the request is first-party
    // or third-party. Without embedding the information into the ID, the
    // endpoints are required to fetch the information from the database. By
    // embedding the information, first-party endpoints could quickly reject
    // requests from third-party applications simply by looking at the
    // application ID provided through the access token claims.
    //
    // The number of attributes could be included is strictly limited.
    pub bitfield: IntegerIdBitfield,
}

impl IntegerId {
    pub fn primitive_size(&self) -> i8 {
        if self.total_bits > 0 {
            //TODO: this should be performed in the compilation phase, and
            // it should also include the size of bitfield.
            if self.total_bits <= self.significant_bits {
                panic!("Invalid directive. The value of total_bits must be larger than the value of significant_bits")
            }
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

//region IntegerIdBitfield

#[derive(Clone, Debug)]
pub struct IntegerIdBitfield {
    pub size: i8,
    pub sub_fields: Vec<IntegerIdBitfieldSubField>,
    pub inherits: Vec<IntegerIdBitfieldInherit>,
}

impl IntegerIdBitfield {
    // default value for size is 'unspecified'
    pub fn size_default() -> i8 {
        -1
    }
}

impl Default for IntegerIdBitfield {
    fn default() -> IntegerIdBitfield {
        IntegerIdBitfield {
            size: IntegerIdBitfield::size_default(),
            sub_fields: Vec::new(),
            inherits: Vec::new(),
        }
    }
}

//endregion

//region IntegerIdBitfieldSubField

#[derive(Clone, Debug)]
pub struct IntegerIdBitfieldSubField {
    pub identifier: String,
    pub documentation: String,
    pub bits: Vec<IntegerIdBitfieldSubFieldBit>,
}

//endregion

//region IntegerIdBitfieldSubFieldBit

#[derive(Clone, Debug)]
pub struct IntegerIdBitfieldSubFieldBit {
    pub index: i8,
    pub set: bool,
}

impl Default for IntegerIdBitfieldSubFieldBit {
    fn default() -> IntegerIdBitfieldSubFieldBit {
        IntegerIdBitfieldSubFieldBit {
            index: -1,
            set: false,
        }
    }
}

//endregion

//region IntegerIdBitfieldInherit

#[derive(Clone, Debug)]
pub struct IntegerIdBitfieldInherit {
    pub host: i8,
    pub size: i8,
}

impl Default for IntegerIdBitfieldInherit {
    fn default() -> IntegerIdBitfieldInherit {
        IntegerIdBitfieldInherit { host: -1, size: -1 }
    }
}

//endregion
