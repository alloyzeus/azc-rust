//

use azml::azml::eid;

//region IntegerIdContext

#[derive(Clone, Gtmpl)]
pub struct IntegerIdContext {
    primitive_size: i8,
    type_name: String,
    flags: Vec<IntegerIdBitFlagContext>,
}

impl From<&eid::IntegerId> for IntegerIdContext {
    fn from(x: &eid::IntegerId) -> IntegerIdContext {
        IntegerIdContext {
            primitive_size: x.primitive_size(),
            type_name: format!("int{}", x.primitive_size()),
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
