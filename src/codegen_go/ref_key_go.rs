//

use azml::azml::ref_key;

//region RefKeyContext

#[derive(Clone, Gtmpl)]
pub struct RefKeyContext {
    pub azis: RefKeyAzisContext,
}

// impl From<&ref_key::RefKey> for RefKeyContext {
//     fn from(x: &ref_key::RefKey) -> RefKeyContext {
//         RefKeyContext {
//             azis: x.azis.into(),
//         }
//     }
// }

impl From<ref_key::RefKey> for RefKeyContext {
    fn from(x: ref_key::RefKey) -> RefKeyContext {
        RefKeyContext {
            azis: x.azis.into(),
        }
    }
}

#[derive(Clone, Gtmpl)]
pub struct RefKeyAzisContext {
    pub prefix: String,
}

impl From<&ref_key::RefKeyAzis> for RefKeyAzisContext {
    fn from(x: &ref_key::RefKeyAzis) -> RefKeyAzisContext {
        RefKeyAzisContext {
            prefix: x.prefix.to_owned(),
        }
    }
}

impl From<ref_key::RefKeyAzis> for RefKeyAzisContext {
    fn from(x: ref_key::RefKeyAzis) -> RefKeyAzisContext {
        RefKeyAzisContext {
            prefix: x.prefix.to_owned(),
        }
    }
}

//endregion
