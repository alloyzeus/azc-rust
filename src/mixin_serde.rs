//

use serde::{Deserialize, Serialize};
use std::convert;

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
