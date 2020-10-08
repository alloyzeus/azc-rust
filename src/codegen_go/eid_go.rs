//

use azml::azml::oid;

//region IntegerIdContext

#[derive(Clone, Gtmpl)]
pub struct IntegerIdContext {
    primitive_size: i8,
    type_name: String,
    flags: Vec<IntegerIdBitFlagContext>,
}

impl From<&oid::IntegerId> for IntegerIdContext {
    fn from(x: &oid::IntegerId) -> IntegerIdContext {
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

impl From<&oid::IntegerIdBitFlag> for IntegerIdBitFlagContext {
    fn from(x: &oid::IntegerIdBitFlag) -> IntegerIdBitFlagContext {
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

impl From<&oid::IntegerIdBitFlagBit> for IntegerIdBitFlagBitContext {
    fn from(x: &oid::IntegerIdBitFlagBit) -> IntegerIdBitFlagBitContext {
        IntegerIdBitFlagBitContext {
            index: x.index,
            set: x.set,
        }
    }
}

//endregion
