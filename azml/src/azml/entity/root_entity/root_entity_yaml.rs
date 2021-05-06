//

use std::convert::{self, TryInto};

use crate::azml::{
    abstract_yaml, attribute, attribute_yaml,
    entity::{entity_id_num_yaml, entity_yaml, lifecycle::lifecycle_yaml},
    mixin, mixin_yaml, yaml,
};

use super::root_entity;

//region RootEntityYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RootEntityYaml {
    id: entity_id_num_yaml::EntityIdYaml,

    #[serde(default)]
    implements: abstract_yaml::AbstractImplementationYaml,

    lifecycle: lifecycle_yaml::LifecycleYaml,

    #[serde(default)]
    mixins: Vec<mixin_yaml::MixinYaml>,

    #[serde(default)]
    service: Option<entity_yaml::EntityServiceYaml>,

    #[serde(default)]
    attributes: Vec<attribute_yaml::AttributeYaml>,
}

impl convert::TryFrom<RootEntityYaml> for root_entity::RootEntity {
    type Error = yaml::Error;

    fn try_from(x: RootEntityYaml) -> Result<Self, Self::Error> {
        Ok(root_entity::RootEntity {
            id: x.id.try_into()?,
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
