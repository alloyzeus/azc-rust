//

use azml::azml::ref_key;

//region RefKeyContext

#[derive(Clone, Gtmpl)]
pub struct RefKeyContext {
    pub azrs: RefKeyAzisContext,
}

// impl From<&ref_key::RefKey> for RefKeyContext {
//     fn from(x: &ref_key::RefKey) -> RefKeyContext {
//         RefKeyContext {
//             azrs: x.azrs.into(),
//         }
//     }
// }

impl From<ref_key::RefKey> for RefKeyContext {
    fn from(x: ref_key::RefKey) -> RefKeyContext {
        RefKeyContext {
            azrs: x.azrs.into(),
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
