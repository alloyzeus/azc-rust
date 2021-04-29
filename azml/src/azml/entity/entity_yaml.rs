//

use std::convert::{self, TryInto};

use crate::azml::{
    abstract_yaml, attribute, attribute_yaml, mixin, mixin_yaml, ref_key_yaml, yaml,
};

use super::{entity, entity_id_num_yaml, lifecycle::lifecycle_yaml};

//region EntityYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EntityYaml {
    id_num: entity_id_num_yaml::EntityIdNumYaml,

    #[serde(default)]
    ref_key: ref_key_yaml::RefKeyYaml,

    #[serde(default)]
    implements: abstract_yaml::AbstractImplementationYaml,

    lifecycle: lifecycle_yaml::LifecycleYaml,

    #[serde(default)]
    mixins: Vec<mixin_yaml::MixinYaml>,

    #[serde(default)]
    service: Option<EntityServiceYaml>,

    #[serde(default)]
    attributes: Vec<attribute_yaml::AttributeYaml>,
}

impl convert::TryFrom<EntityYaml> for entity::Entity {
    type Error = yaml::Error;

    fn try_from(x: EntityYaml) -> Result<Self, Self::Error> {
        Ok(entity::Entity {
            id_num: x.id_num.try_into()?,
            ref_key: x.ref_key.try_into()?,
            implements: x.implements.try_into()?,
            lifecycle: x.lifecycle.try_into()?,
            mixins: x
                .mixins
                .into_iter()
                .map(|x| mixin::Mixin::try_from(x))
                .collect::<Result<Vec<mixin::Mixin>, _>>()?,
            service: if let Some(service) = x.service.clone() {
                Some(service.into())
            } else {
                None
            },
            attributes: x
                .attributes
                .iter()
                .map(|attr| attribute::Attribute::try_from(attr).unwrap())
                .collect(),
        })
    }
}

//endregion

//region EntityServiceYaml

#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
struct EntityServiceYaml {
    #[serde(default)]
    documentation: String,

    #[serde(default)]
    enabled: bool,
}

impl From<&EntityServiceYaml> for entity::EntityService {
    fn from(x: &EntityServiceYaml) -> entity::EntityService {
        entity::EntityService {
            documentation: x.documentation.to_owned(),
            enabled: x.enabled,
        }
    }
}

impl From<EntityServiceYaml> for entity::EntityService {
    fn from(x: EntityServiceYaml) -> entity::EntityService {
        (&x).into()
    }
}

//endregion
