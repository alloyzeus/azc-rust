//

use serde::{Deserialize, Serialize};
use std::{convert, convert::TryInto};

use crate::{azyaml, mixin, mixins::ownership, mixins::ownership_serde};

#[derive(Serialize, Deserialize)]
pub struct MixinSerde {
    kind: String,

    parameters: azyaml::Value,
}

impl convert::TryFrom<MixinSerde> for mixin::Mixin {
    type Error = azyaml::Error;

    fn try_from(x: MixinSerde) -> Result<Self, Self::Error> {
        match x.kind.as_str() {
            "Ownership" => {
                let params: Option<ownership_serde::OwnershipSerde> = azyaml::from_value(x.parameters)?;
                Ok(mixin::Mixin {
                    parameters: if let Some(p) = params {
                        Some(Box::new(ownership::Ownership::try_from(p)?))
                    } else {
                        None
                    },
                })
            }
            _ => Ok(mixin::Mixin{parameters: None})
            // _ => Err(azyaml::Error::Msg(format!(
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
    <U as convert::TryFrom<T>>::Error: Into<azyaml::Error>,
    <U as convert::TryFrom<T>>::Error: std::fmt::Debug,
{
    type Error = azyaml::Error;

    fn try_from(x: MixinFieldSerde<T>) -> Result<Self, Self::Error> {
        Ok(mixin::MixinField {
            overridable: x.overridable,
            value: x.value.try_into().unwrap(), // U::try_from(x.value)?,
        })
    }
}
