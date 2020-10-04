//

use std::fmt;

//region Symbol

#[derive(Clone, Debug)]
pub struct Symbol {
    pub identifier: String,
    pub definition: Box<dyn SymbolDefinition>,

    // Documentation field provides a documentation for the symbol.
    // Generally, the value of this field will be used as the documentation
    // for the generated symbol, usually in form of comment.
    //
    // It's recommended to make this documentation starts with a complete
    // sentence that begins with the name of the symbol it describes with
    // an optional leading article.
    pub documentation: String,
    //TODO: naming options for the identifier. we can explicitly define the
    // standard naming option, which required to be consistent across
    // implementations e.g., for identifier in encoded data.
    // We can also explicitly define the name in different
    // cases, SymbolPascalCase, symbolCamelCase, symbol_snake_case,
    // symbol-kebab-case, symbolflatcase, SYMBOL_MACRO_CASE, etc. These
    // letter case options affect only for generating the codes; they won't
    // affect the identifiers for inter-process communications.
    // We will also allow the letter-casing options based on the
    // generators and usages, for class/struct names, for file names. These
    // options will be useful for conformance with language's
    // naming convention, e.g., HttpRequest in contrast to HTTPRequest,
    // HttpRequest.java in contrast to http_request.rs .
}

//endregion

//region SymbolDefinition

pub trait SymbolDefinition: mopa::Any + SymbolDefinitionClone + fmt::Debug {}

mopafy!(SymbolDefinition);

pub trait SymbolDefinitionClone {
    fn clone_box(&self) -> Box<dyn SymbolDefinition>;
}

impl<T> SymbolDefinitionClone for T
where
    T: SymbolDefinition + Clone,
{
    fn clone_box(&self) -> Box<dyn SymbolDefinition> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SymbolDefinition> {
    fn clone(&self) -> Box<dyn SymbolDefinition> {
        self.clone_box()
    }
}

//endregion

//region SymbolRef

#[derive(Clone, Debug)]
pub struct SymbolRef {
    pub package_identifier: String,
    pub symbol_name: String,
}

impl From<String> for SymbolRef {
    fn from(s: String) -> SymbolRef {
        (&s).into()
    }
}

impl From<&String> for SymbolRef {
    fn from(s: &String) -> SymbolRef {
        let parts: Vec<&str> = s.rsplitn(2, ".").collect();
        if parts.len() == 2 {
            SymbolRef {
                package_identifier: parts[1].to_owned(),
                symbol_name: parts[0].to_owned(),
            }
        } else {
            SymbolRef {
                package_identifier: "".to_owned(),
                symbol_name: parts[0].to_owned(),
            }
        }
    }
}

impl From<SymbolRef> for String {
    fn from(s: SymbolRef) -> String {
        (&s).into()
    }
}

impl From<&SymbolRef> for String {
    fn from(s: &SymbolRef) -> String {
        if s.package_identifier.is_empty() {
            s.symbol_name.to_owned()
        } else {
            format!("{}.{}", s.package_identifier, s.symbol_name)
        }
    }
}

//endregion
