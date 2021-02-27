//

use azml::azml::ref_key;

//region RefKeyContext

#[derive(Clone, Gtmpl)]
pub struct RefKeyContext {
    pub azer_text: RefKeyAzerTextContext,
}

// impl From<&ref_key::RefKey> for RefKeyContext {
//     fn from(x: &ref_key::RefKey) -> RefKeyContext {
//         RefKeyContext {
//             azer_text: x.azer_text.into(),
//         }
//     }
// }

impl From<ref_key::RefKey> for RefKeyContext {
    fn from(x: ref_key::RefKey) -> RefKeyContext {
        RefKeyContext {
            azer_text: x.azer_text.into(),
        }
    }
}

#[derive(Clone, Gtmpl)]
pub struct RefKeyAzerTextContext {
    pub prefix: String,
}

impl From<&ref_key::RefKeyAzerText> for RefKeyAzerTextContext {
    fn from(x: &ref_key::RefKeyAzerText) -> RefKeyAzerTextContext {
        RefKeyAzerTextContext {
            prefix: x.prefix.to_owned(),
        }
    }
}

impl From<ref_key::RefKeyAzerText> for RefKeyAzerTextContext {
    fn from(x: ref_key::RefKeyAzerText) -> RefKeyAzerTextContext {
        RefKeyAzerTextContext {
            prefix: x.prefix.to_owned(),
        }
    }
}

//endregion
