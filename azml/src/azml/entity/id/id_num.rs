//

use crate::azml::symbol;

// Id-num is the number part of an id.

//region IdNum

#[derive(Clone, Debug)]
pub struct IdNum {
    pub definition: Box<dyn IdNumDefinition>,
}

//endregion

//region IdNumDefinition

pub trait IdNumDefinition: mopa::Any + IdNumDefinitionClone + std::fmt::Debug {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef>;
}

mopafy!(IdNumDefinition);

pub trait IdNumDefinitionClone {
    fn clone_boxed_entity_id_num_definition(&self) -> Box<dyn IdNumDefinition>;
}

impl<T> IdNumDefinitionClone for T
where
    T: IdNumDefinition + Clone,
{
    fn clone_boxed_entity_id_num_definition(&self) -> Box<dyn IdNumDefinition> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn IdNumDefinition> {
    fn clone(&self) -> Box<dyn IdNumDefinition> {
        self.clone_boxed_entity_id_num_definition()
    }
}

//endregion

//region IntegerIdNum

#[derive(Clone, Debug)]
pub struct IntegerIdNum {
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
    // identifier_bits.
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
    // 0 in the context of user id-num could be used to indicate nobody.
    pub identifier_bits: i8,

    // Additional attributes (flags) encoding. these attributes are
    // part of the identity for the entity's lifetime.
    //
    // As an example use case, there's two types of application, which is
    // first-part and third-party. First-party applications could access
    // more API than that of available to third parties. These API will need
    // to check whether the application which made the request is first-party
    // or third-party. Without embedding the information into the id-num, the
    // endpoints are required to fetch the information from the database. By
    // embedding the information, first-party endpoints could quickly reject
    // requests from third-party applications simply by looking at the
    // application id-num provided through the access token claims.
    //
    // The number of attributes could be included is strictly limited.
    pub bitfield: IntegerIdNumBitfield,
}

impl IntegerIdNum {
    pub fn primitive_size(&self) -> i8 {
        if self.total_bits > 0 {
            //TODO: this should be performed in the compilation phase, and
            // it should also include the size of bitfield.
            if self.total_bits <= self.identifier_bits {
                panic!("Invalid directive. The value of total_bits must be larger than the value of identifier_bits")
            }
            self.total_bits
        } else {
            match self.identifier_bits {
                d if d < 16 => 16,
                d if d < 32 => 32,
                d if d < 64 => 64,
                _ => panic!(
                    "Unsupported bits value {} (bits value must be smaller than 64)",
                    self.identifier_bits
                ),
            }
        }
    }
}

impl IdNumDefinition for IntegerIdNum {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        Vec::new()
    }
}

//endregion

//region IntegerIdNumBitfield

#[derive(Clone, Debug)]
pub struct IntegerIdNumBitfield {
    pub size: i8,
    pub sub_fields: Vec<IntegerIdNumBitfieldSubField>,
    pub inherits: Vec<IntegerIdNumBitfieldInherit>,
}

impl IntegerIdNumBitfield {
    // default value for size is 'unspecified'
    pub fn size_default() -> i8 {
        -1
    }
}

impl Default for IntegerIdNumBitfield {
    fn default() -> Self {
        Self {
            size: IntegerIdNumBitfield::size_default(),
            sub_fields: Vec::new(),
            inherits: Vec::new(),
        }
    }
}

//endregion

//region IntegerIdNumBitfieldSubField

#[derive(Clone, Debug)]
pub struct IntegerIdNumBitfieldSubField {
    pub identifier: String,
    pub documentation: String,
    pub size: i8,
    pub values: Vec<IntegerIdNumBitfieldSubFieldValue>,
}

//endregion

#[derive(Clone, Debug)]
pub struct IntegerIdNumBitfieldSubFieldValue {
    pub identifier: String,
    pub documentation: String,
    pub sub_fields: Vec<IntegerIdNumBitfieldSubField>,
}

//region IntegerIdNumBitfieldSubFieldBit

#[derive(Clone, Debug)]
pub struct IntegerIdNumBitfieldSubFieldBit {
    pub index: i8,
    pub set: bool,
}

impl Default for IntegerIdNumBitfieldSubFieldBit {
    fn default() -> Self {
        Self {
            index: -1,
            set: false,
        }
    }
}

//endregion

//region IntegerIdNumBitfieldInherit

#[derive(Clone, Debug)]
pub struct IntegerIdNumBitfieldInherit {
    pub host: i8,
    pub size: i8,
}

impl Default for IntegerIdNumBitfieldInherit {
    fn default() -> Self {
        Self { host: -1, size: -1 }
    }
}

//endregion
