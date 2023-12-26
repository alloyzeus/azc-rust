//

use azml::azml::symbol;

//region SymbolRefContext

#[derive(Clone, Gtmpl)]
pub struct SymbolRefContext {
    package_identifier: String,
    symbol_name: String,
    fqn: String,
    is_slice: bool,
}

impl From<&symbol::SymbolRef> for SymbolRefContext {
    fn from(r: &symbol::SymbolRef) -> Self {
        let fqn = if r.package_identifier.is_empty() {
            r.symbol_name.to_owned()
        } else {
            format!("{}.{}", r.package_identifier, r.symbol_name)
        };
        let fqn = if r.is_slice {
            format!("[]{}", fqn)
        } else {
            fqn
        };
        Self {
            package_identifier: r.package_identifier.to_owned(),
            //TODO: should be RefObject instead of RefKey.
            symbol_name: if r.is_reference {
                format!("{}ID", r.symbol_name)
            } else {
                r.symbol_name.to_owned()
            },
            fqn,
            is_slice: r.is_slice,
        }
    }
}

// impl From<SymbolRefContext> for String {
//     fn from(s: SymbolRefContext) -> Self {
//         (&s).into()
//     }
// }

// impl From<&SymbolRefContext> for String {
//     fn from(s: &SymbolRefContext) -> Self {
//         if s.package_identifier.is_empty() {
//             s.symbol_name.to_owned()
//         } else {
//             format!("{}.{}", s.package_identifier, s.symbol_name)
//         }
//     }
// }

// impl From<SymbolRefContext> for gtmpl::Value {
//     fn from(s: SymbolRefContext) -> Self {
//         (&s).into()
//     }
// }

// impl From<&SymbolRefContext> for gtmpl::Value {
//     fn from(s: &SymbolRefContext) -> Self {
//         Self::String(s.into())
//     }
// }

//endregion
