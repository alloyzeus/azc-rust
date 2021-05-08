//

use std::convert::{self, TryInto};

use crate::azml::{
    attribute, attribute_yaml,
    entity::{abstract_, abstract_yaml, entity_yaml, id::id_yaml, lifecycle::lifecycle_yaml},
    mixin, mixin_yaml, yaml,
};

use super::root_entity;

//region RootEntityYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RootEntityYaml {
    id: id_yaml::IdYaml,

    #[serde(default)]
    implements: Vec<abstract_yaml::AbstractImplementationYaml>,

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
            implements: x
                .implements
                .iter()
                .map(|x| abstract_::AbstractImplementation::try_from(x))
                .collect::<Result<Vec<abstract_::AbstractImplementation>, _>>()?,
            lifecycle: x.lifecycle.try_into()?,
            mixins: x
                .mixins
                .iter()
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
                .map(|attr| attribute::Attribute::try_from(attr))
                .collect::<Result<Vec<attribute::Attribute>, _>>()?,
        })
    }
}

//endregion
