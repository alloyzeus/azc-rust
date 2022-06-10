//

use azml::azml::symbol;

//region SymbolRefContext

#[derive(Clone)]
pub struct SymbolRefContext {
    package_identifier: String,
    symbol_name: String,
}

impl From<&symbol::SymbolRef> for SymbolRefContext {
    fn from(r: &symbol::SymbolRef) -> SymbolRefContext {
        SymbolRefContext {
            package_identifier: r.package_identifier.to_owned(),
            //TODO: should be RefObject instead of RefKey.
            symbol_name: if r.is_reference {
                format!("{}ID", r.symbol_name)
            } else {
                r.symbol_name.to_owned()
            },
        }
    }
}

impl From<SymbolRefContext> for String {
    fn from(s: SymbolRefContext) -> String {
        (&s).into()
    }
}

impl From<&SymbolRefContext> for String {
    fn from(s: &SymbolRefContext) -> String {
        if s.package_identifier.is_empty() {
            s.symbol_name.to_owned()
        } else {
            format!("{}.{}", s.package_identifier, s.symbol_name)
        }
    }
}

impl From<SymbolRefContext> for gtmpl_value::Value {
    fn from(s: SymbolRefContext) -> gtmpl_value::Value {
        (&s).into()
    }
}

impl From<&SymbolRefContext> for gtmpl_value::Value {
    fn from(s: &SymbolRefContext) -> gtmpl_value::Value {
        gtmpl_value::Value::String(s.into())
    }
}

//endregion
