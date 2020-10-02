//

use std::{
    collections::HashMap,
    convert::{self, TryInto},
};

use crate::azml::{
    attribute, attribute_yaml,
    entity::{entity, entity_id_yaml},
    mixin, mixin_yaml, yaml,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EntityYaml {
    id: entity_id_yaml::EntityIdYaml,

    creation: EntityCreationYaml,
    mixins: Vec<mixin_yaml::MixinYaml>,

    #[serde(default)]
    service: Option<EntityServiceYaml>,

    #[serde(default)]
    attributes: HashMap<String, attribute_yaml::AttributeYaml>,
}

impl convert::TryFrom<EntityYaml> for entity::Entity {
    type Error = yaml::Error;

    fn try_from(x: EntityYaml) -> Result<Self, Self::Error> {
        Ok(entity::Entity {
            id: x.id.try_into()?,
            creation: x.creation.try_into()?,
            mixins: x
                .mixins
                .into_iter()
                .map(|x| mixin::Mixin::try_from(x))
                .collect::<Result<Vec<mixin::Mixin>, _>>()?,
            service: if let Some(service) = x.service {
                Some(service.into())
            } else {
                None
            },
            attributes: x
                .attributes
                .iter()
                .map(|(k, v)| (k.to_owned(), attribute::Attribute::try_from(v).unwrap()))
                .collect::<HashMap<String, attribute::Attribute>>(),
        })
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct EntityCreationYaml {
    #[serde(default)]
    documentation: String,
}

impl convert::TryFrom<EntityCreationYaml> for entity::EntityCreation {
    type Error = yaml::Error;

    fn try_from(x: EntityCreationYaml) -> Result<Self, Self::Error> {
        Ok(entity::EntityCreation {
            documentation: x.documentation,
        })
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct EntityServiceYaml {
    #[serde(default)]
    documentation: String,

    #[serde(default)]
    enabled: bool,
}

impl From<EntityServiceYaml> for entity::EntityService {
    fn from(x: EntityServiceYaml) -> entity::EntityService {
        entity::EntityService {
            documentation: x.documentation,
            enabled: x.enabled,
        }
    }
}
