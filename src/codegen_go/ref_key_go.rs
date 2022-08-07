//

use azml::azml::entity::id::ref_key;

//region RefKeyContext

#[derive(Clone, Gtmpl)]
pub struct RefKeyContext {
    pub azid_text: RefKeyAzidTextContext,
}

// impl From<&ref_key::RefKey> for RefKeyContext {
//     fn from(x: &ref_key::RefKey) -> Self {
//         Self {
//             azid_text: x.azid_text.into(),
//         }
//     }
// }

impl From<ref_key::RefKey> for RefKeyContext {
    fn from(x: ref_key::RefKey) -> Self {
        Self {
            azid_text: x.azid_text.into(),
        }
    }
}

#[derive(Clone, Gtmpl)]
pub struct RefKeyAzidTextContext {
    pub prefix: String,
}

impl From<&ref_key::RefKeyAzidText> for RefKeyAzidTextContext {
    fn from(x: &ref_key::RefKeyAzidText) -> Self {
        Self {
            prefix: x.prefix.to_owned(),
        }
    }
}

impl From<ref_key::RefKeyAzidText> for RefKeyAzidTextContext {
    fn from(x: ref_key::RefKeyAzidText) -> Self {
        Self {
            prefix: x.prefix.to_owned(),
        }
    }
}

//endregion
