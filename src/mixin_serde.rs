//

use serde::{Deserialize, Serialize};
use std::{convert, convert::TryInto};

use crate::{base::azml, mixin, mixins::ownership_serde};

#[derive(Serialize, Deserialize)]
pub struct MixinSerde {
    kind: String,

    //TODO: required
    #[serde(default)]
    parameters: azml::Value,
}

impl convert::TryFrom<MixinSerde> for mixin::Mixin {
    type Error = azml::Error;

    fn try_from(x: MixinSerde) -> Result<Self, Self::Error> {
        match x.kind.as_str() {
            "Ownable" => {
                let params: Option<ownership_serde::OwnableSerde> = azml::from_value(x.parameters)?;
                Ok(mixin::Mixin {
                    kind: x.kind,
                    parameters: if let Some(p) = params {
                        Some(Box::new(p))
                    } else {
                        None
                    },
                })
            }
            _ => Ok(mixin::Mixin{kind: x.kind, parameters: None})
            // _ => Err(azml::Error::Msg(format!(
            //     r#"Unrecognized mixin `{}`"#,
            //     x.kind
            // ))),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MixinFieldSerde<T> {
    #[serde(default)]
    pub overridable: bool,

    pub value: T,
}

impl<T, U> convert::TryFrom<MixinFieldSerde<T>> for mixin::MixinField<U>
where
    U: convert::TryFrom<T>,
    <U as convert::TryFrom<T>>::Error: Into<azml::Error>,
    <U as convert::TryFrom<T>>::Error: std::fmt::Debug,
{
    type Error = azml::Error;

    fn try_from(x: MixinFieldSerde<T>) -> Result<Self, Self::Error> {
        Ok(mixin::MixinField {
            overridable: x.overridable,
            value: x.value.try_into().unwrap(), // U::try_from(x.value)?,
        })
    }
}
