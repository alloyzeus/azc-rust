//

use serde::{Deserialize, Serialize};

use crate::{base::azml, mixin, mixins::ownership_serde};

#[derive(Serialize, Deserialize)]
pub struct MixinSerde {
    kind: String,

    //TODO: required
    #[serde(default)]
    parameters: azml::Value,
}

impl From<MixinSerde> for mixin::Mixin {
    fn from(x: MixinSerde) -> mixin::Mixin {
        match x.kind.as_str() {
            "Ownable" => {
                let params: Option<ownership_serde::OwnableSerde> = if x.parameters.is_mapping() {
                    azml::from_value(x.parameters).unwrap_or(None)
                } else {
                    None
                };
                mixin::Mixin {
                    kind: x.kind,
                    parameters: if let Some(p) = params {
                        Some(Box::new(p))
                    } else {
                        None
                    },
                }
            }
            _ => mixin::Mixin {
                kind: x.kind,
                parameters: None,
            },
        }
    }
}
