//

use azml::azml::eid;

//region IntegerIdContext

#[derive(Clone, Gtmpl)]
pub struct IntegerIdContext {
    primitive_size: i8,
    type_name: String,
    significant_bits: i8,
    significant_bits_mask: String,
    significant_bits_mask_bin: String,
    bitfield: IntegerIdBitfieldContext,
}

impl From<&eid::IntegerId> for IntegerIdContext {
    fn from(x: &eid::IntegerId) -> IntegerIdContext {
        IntegerIdContext {
            primitive_size: x.primitive_size(),
            type_name: format!("int{}", x.primitive_size()),
            significant_bits: x.significant_bits,
            significant_bits_mask: significant_bit_mask_hex(x.primitive_size(), x.significant_bits),
            significant_bits_mask_bin: significant_bit_mask_bin(x.primitive_size(), x.significant_bits),
            // Minus 2: one for making it zero-based index, one for skipping the most-significant bit (the sign bit)
            bitfield: IntegerIdBitfieldContext::from(&x.bitfield, x.primitive_size() - 2),
        }
    }
}

fn significant_bit_mask_hex(bit_size: i8, significant_size: i8) -> String {
    let mut v: u64 = 0;
    for i in 0..bit_size {
        if i < significant_size {
            v |= 1 << i;
        }
    }
    format!("0x{:x}", v)
}

fn significant_bit_mask_bin(bit_size: i8, significant_size: i8) -> String {
    let mut v: u64 = 0;
    for i in 0..bit_size {
        if i < significant_size {
            v |= 1 << i;
        }
    }
    format!("0b{:b}", v)
}

//endregion

//region IntegerIdBitfieldContext

#[derive(Clone, Gtmpl)]
pub struct IntegerIdBitfieldContext {
    pub sub_fields: Vec<IntegerIdBitfieldSubFieldContext>,
}

impl IntegerIdBitfieldContext {
    fn from(x: &eid::IntegerIdBitfield, index_offset: i8) -> IntegerIdBitfieldContext {
        IntegerIdBitfieldContext {
            sub_fields: x
                .sub_fields
                .iter()
                .map(|sub_field| IntegerIdBitfieldSubFieldContext::from(sub_field, index_offset))
                .collect(),
        }
    }
}

impl From<&eid::IntegerIdBitfield> for IntegerIdBitfieldContext {
    fn from(x: &eid::IntegerIdBitfield) -> IntegerIdBitfieldContext {
        IntegerIdBitfieldContext {
            sub_fields: x.sub_fields.iter().map(|x| x.into()).collect(),
        }
    }
}

//endregion

//region IntegerIdBitfieldSubFieldContext

#[derive(Clone, Gtmpl)]
pub struct IntegerIdBitfieldSubFieldContext {
    identifier: String,
    doc_lines: Vec<String>,
    bits: Vec<IntegerIdBitfieldSubFieldBitContext>,
}

impl IntegerIdBitfieldSubFieldContext {
    fn from(
        x: &eid::IntegerIdBitfieldSubField,
        index_offset: i8,
    ) -> IntegerIdBitfieldSubFieldContext {
        IntegerIdBitfieldSubFieldContext {
            identifier: x.identifier.to_owned(),
            doc_lines: x.documentation.lines().map(|x| x.to_owned()).collect(),
            bits: x
                .bits
                .iter()
                .map(|bit| IntegerIdBitfieldSubFieldBitContext::from(bit, index_offset))
                .collect(),
        }
    }
}

impl From<&eid::IntegerIdBitfieldSubField> for IntegerIdBitfieldSubFieldContext {
    fn from(x: &eid::IntegerIdBitfieldSubField) -> IntegerIdBitfieldSubFieldContext {
        IntegerIdBitfieldSubFieldContext {
            identifier: x.identifier.to_owned(),
            doc_lines: x.documentation.lines().map(|x| x.to_owned()).collect(),
            bits: x.bits.iter().map(|x| x.into()).collect(),
        }
    }
}

//endregion

//region IntegerIdBitFieldSubFieldBitContext

#[derive(Clone, Gtmpl)]
pub struct IntegerIdBitfieldSubFieldBitContext {
    pub index: i8,
    pub set: bool,
}

impl IntegerIdBitfieldSubFieldBitContext {
    fn from(
        x: &eid::IntegerIdBitfieldSubFieldBit,
        offset: i8,
    ) -> IntegerIdBitfieldSubFieldBitContext {
        IntegerIdBitfieldSubFieldBitContext {
            index: offset - x.index,
            set: x.set,
        }
    }
}

impl From<&eid::IntegerIdBitfieldSubFieldBit> for IntegerIdBitfieldSubFieldBitContext {
    fn from(x: &eid::IntegerIdBitfieldSubFieldBit) -> IntegerIdBitfieldSubFieldBitContext {
        IntegerIdBitfieldSubFieldBitContext {
            index: x.index,
            set: x.set,
        }
    }
}

//endregion

#[derive(Clone, Gtmpl)]
pub struct IntegerIdTextEncodingContext {
    pub prefix: String,
    pub encoding: String,
}

impl From<&eid::IntegerIdTextEncoding> for IntegerIdTextEncodingContext {
    fn from(x: &eid::IntegerIdTextEncoding) -> IntegerIdTextEncodingContext {
        IntegerIdTextEncodingContext {
            prefix: x.prefix.to_owned(),
            encoding: x.encoding.to_owned(),
        }
    }
}
