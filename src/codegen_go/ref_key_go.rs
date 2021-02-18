//

use azml::azml::ref_key;

//region RefKeyContext

#[derive(Clone, Gtmpl)]
pub struct RefKeyContext {
    pub string_prefix: String,
}

impl From<&ref_key::RefKey> for RefKeyContext {
    fn from(x: &ref_key::RefKey) -> RefKeyContext {
        RefKeyContext {
            string_prefix: x.identifier.to_owned(),
        }
    }
}

//endregion
