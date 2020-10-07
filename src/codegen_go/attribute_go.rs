//

use azml::azml::attribute;

use crate::codegen_go::symbol_go;

#[derive(Clone, Gtmpl)]
pub struct AttributeContext {
    pub identifier: String,
    pub kind: symbol_go::SymbolRefContext,
    pub final_: bool,
}

impl From<&attribute::Attribute> for AttributeContext {
    fn from(x: &attribute::Attribute) -> AttributeContext {
        AttributeContext {
            identifier: x.identifier.to_owned(),
            kind: (&x.kind).into(),
            final_: x.final_,
        }
    }
}

impl From<attribute::Attribute> for AttributeContext {
    fn from(x: attribute::Attribute) -> AttributeContext {
        (&x).into()
    }
}
