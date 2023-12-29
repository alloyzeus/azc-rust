//

use crate::convert_case::{Case, Casing};

use azml::azml::attribute;

use super::symbol_go;

#[derive(Clone, Gtmpl)]
pub struct AttributeContext {
    pub identifier: String,
    pub kind: symbol_go::SymbolRefContext,
    pub finality: String,
    pub db_col_name: String,
}

impl From<&attribute::Attribute> for AttributeContext {
    fn from(x: &attribute::Attribute) -> Self {
        let db_col_name = if x.name_options.snake_case.is_empty() {
            x.name.to_case(Case::Snake)
        } else {
            x.name_options.snake_case.to_owned()
        };
        Self {
            identifier: x.name.to_owned(),
            kind: (&x.kind).into(),
            finality: (&x.finality).into(),
            db_col_name,
        }
    }
}

impl From<attribute::Attribute> for AttributeContext {
    fn from(x: attribute::Attribute) -> Self {
        (&x).into()
    }
}
