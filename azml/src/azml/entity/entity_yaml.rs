//

use std::convert::{self, TryInto};

use crate::azml::{
    attribute, attribute_yaml,
    entity::{entity, entity_id_yaml},
    mixin, mixin_yaml, ref_key_yaml, yaml,
};

//region EntityYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EntityYaml {
    id: entity_id_yaml::EntityIdYaml,

    #[serde(default)]
    ref_key: ref_key_yaml::RefKeyYaml,

    creation: EntityCreationYaml,
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
            id: x.id.try_into()?,
            ref_key: x.ref_key.try_into()?,
            creation: x.creation.try_into()?,
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

impl Default for ref_key_yaml::RefKeyYaml {
    fn default() -> ref_key_yaml::RefKeyYaml {
        ref_key_yaml::RefKeyYaml {}
    }
}

//region EntityCreationYaml

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct EntityCreationYaml {
    #[serde(default)]
    documentation: String,

    allow_cross_process_callers: bool,
}

impl convert::TryFrom<&EntityCreationYaml> for entity::EntityCreation {
    type Error = yaml::Error;

    fn try_from(x: &EntityCreationYaml) -> Result<Self, Self::Error> {
        Ok(entity::EntityCreation {
            documentation: x.documentation.to_owned(),
            allow_cross_process_callers: x.allow_cross_process_callers,
        })
    }
}

impl convert::TryFrom<EntityCreationYaml> for entity::EntityCreation {
    type Error = yaml::Error;

    fn try_from(x: EntityCreationYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
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
