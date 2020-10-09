//

use azml::azml::eid;

//region IntegerIdContext

#[derive(Clone, Gtmpl)]
pub struct IntegerIdContext {
    primitive_size: i8,
    type_name: String,
    bitfield: IntegerIdBitfieldContext,
    flags: Vec<IntegerIdBitFlagContext>,
}

impl From<&eid::IntegerId> for IntegerIdContext {
    fn from(x: &eid::IntegerId) -> IntegerIdContext {
        IntegerIdContext {
            primitive_size: x.primitive_size(),
            type_name: format!("int{}", x.primitive_size()),
            // Minus 2: one for making it zero-based index, one for skipping the first bit (sign-bit)
            bitfield: IntegerIdBitfieldContext::from(&x.bitfield, x.primitive_size() - 2),
            flags: x.flags.iter().map(|x| x.into()).collect(),
        }
    }
}

//endregion

//region IntegerIdBitFlagContext

#[derive(Clone, Gtmpl)]
pub struct IntegerIdBitFlagContext {
    pub identifier: String,
    pub doc_lines: Vec<String>,
    pub bits: Vec<IntegerIdBitFlagBitContext>,
}

impl From<&eid::IntegerIdBitFlag> for IntegerIdBitFlagContext {
    fn from(x: &eid::IntegerIdBitFlag) -> IntegerIdBitFlagContext {
        IntegerIdBitFlagContext {
            identifier: x.identifier.to_owned(),
            doc_lines: x.documentation.lines().map(|x| x.to_owned()).collect(),
            bits: x.bits.iter().map(|x| x.into()).collect(),
        }
    }
}

//endregion

//region IntegerIdBitFlagContext

#[derive(Clone, Gtmpl)]
pub struct IntegerIdBitFlagBitContext {
    pub index: i8,
    pub set: bool,
}

impl From<&eid::IntegerIdBitFlagBit> for IntegerIdBitFlagBitContext {
    fn from(x: &eid::IntegerIdBitFlagBit) -> IntegerIdBitFlagBitContext {
        IntegerIdBitFlagBitContext {
            index: x.index,
            set: x.set,
        }
    }
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
