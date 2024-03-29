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
    //TODO: singular and plural. but we'll only need to define the
    // term for singular and the other for collection. e.g., entity and
    // entities. the collection option will be used for, e.g., url path
    // and table names.
}

//endregion

//region SymbolDefinition

pub trait SymbolDefinition: mopa::Any + SymbolDefinitionClone + fmt::Debug {
    fn collect_symbol_refs(&self) -> Vec<SymbolRef>;
}

mopafy!(SymbolDefinition);

pub trait SymbolDefinitionClone {
    fn clone_boxed_symbol_definition(&self) -> Box<dyn SymbolDefinition>;
}

impl<T> SymbolDefinitionClone for T
where
    T: SymbolDefinition + Clone,
{
    fn clone_boxed_symbol_definition(&self) -> Box<dyn SymbolDefinition> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SymbolDefinition> {
    fn clone(&self) -> Box<dyn SymbolDefinition> {
        self.clone_boxed_symbol_definition()
    }
}

//endregion

//region SymbolRef

#[derive(Clone, Debug)]
pub struct SymbolRef { // TODO: rename to TypeRef
    pub package_identifier: String,
    pub symbol_name: String,
    pub is_slice: bool,

    //TODO: this should be optional. if the target symbol is an entity,
    // we defaults to reference.
    pub is_reference: bool,
}

impl From<String> for SymbolRef {
    fn from(s: String) -> Self {
        (&s).into()
    }
}

impl From<&String> for SymbolRef {
    fn from(s: &String) -> Self {
        let is_slice = s.starts_with("list of ");
        let s = if let Some(x) = s.strip_prefix("list of ") {
            x.to_string()
        } else {
            s.to_owned()
        };
        // might not need to explicitly declare the reference.
        let is_reference = s.starts_with("@");
        let s = if let Some(x) = s.strip_prefix("@") {
            x.to_string()
        } else {
            s.to_owned()
        };
        let parts: Vec<&str> = s.rsplitn(2, ".").collect();
        if parts.len() == 2 {
            Self {
                package_identifier: parts[1].to_owned(),
                symbol_name: parts[0].to_owned(),
                is_slice,
                is_reference,
            }
        } else {
            Self {
                package_identifier: "".to_owned(),
                symbol_name: s.to_owned(),
                is_slice,
                is_reference,
            }
        }
    }
}

impl From<SymbolRef> for String {
    fn from(s: SymbolRef) -> Self {
        (&s).into()
    }
}

impl From<&SymbolRef> for String {
    fn from(s: &SymbolRef) -> Self {
        if s.package_identifier.is_empty() {
            s.symbol_name.to_owned()
        } else {
            format!("{}.{}", s.package_identifier, s.symbol_name)
        }
    }
}

//endregion
