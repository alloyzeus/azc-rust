//

use std::convert::{self, TryInto};

use super::{
    entity::{
        lifecycle::deletion::{deletion, deletion_yaml},
        ownership::{self, ownership_yaml},
    },
    mixin, yaml,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MixinYaml {
    kind: String,

    parameters: yaml::Value,
}

impl convert::TryFrom<MixinYaml> for mixin::Mixin {
    type Error = yaml::Error;

    fn try_from(x: MixinYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

impl convert::TryFrom<&MixinYaml> for mixin::Mixin {
    type Error = yaml::Error;

    fn try_from(x: &MixinYaml) -> Result<Self, Self::Error> {
        match x.kind.as_str() {
            "Deletion" => {
                let params: Option<deletion_yaml::DeletionYaml> = yaml::from_value(x.parameters.clone())?;
                Ok(mixin::Mixin {
                    definition: if let Some(p) = params {
                        Some(Box::new(deletion::Deletion::try_from(p)?))
                    } else {
                        None
                    },
                })
            }
            "Ownership" => {
                let params: Option<ownership_yaml::OwnershipYaml> = yaml::from_value(x.parameters.clone())?;
                Ok(mixin::Mixin {
                    definition: if let Some(p) = params {
                        Some(Box::new(ownership::Ownership::try_from(p)?))
                    } else {
                        None
                    },
                })
            }
            _ => Ok(mixin::Mixin{definition: None})
            // _ => Err(yaml::Error::Msg(format!(
            //     r#"Unrecognized mixin `{}`"#,
            //     x.kind
            // ))),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MixinFieldYaml<T> {
    #[serde(default)]
    pub overridable: bool,

    pub value: T,
}

impl<T, U> convert::TryFrom<MixinFieldYaml<T>> for mixin::MixinField<U>
where
    U: convert::TryFrom<T>,
    <U as convert::TryFrom<T>>::Error: Into<yaml::Error>,
    <U as convert::TryFrom<T>>::Error: std::fmt::Debug,
{
    type Error = yaml::Error;

    fn try_from(x: MixinFieldYaml<T>) -> Result<Self, Self::Error> {
        Ok(mixin::MixinField {
            overridable: x.overridable,
            value: x.value.try_into().unwrap(), // U::try_from(x.value)?,
        })
    }
}
