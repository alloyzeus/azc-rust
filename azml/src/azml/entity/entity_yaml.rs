//

use std::convert::{self, TryInto};

use crate::azml::{
    abstract_yaml, attribute, attribute_yaml,
    entity::{entity, entity_id_num_yaml},
    error, mixin, mixin_yaml, ref_key_yaml, yaml,
};

//region EntityYaml

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EntityYaml {
    id_num: entity_id_num_yaml::EntityIdNumYaml,

    #[serde(default)]
    ref_key: ref_key_yaml::RefKeyYaml,

    #[serde(default)]
    implements: abstract_yaml::AbstractImplementationYaml,

    lifecycle: EntityLifecycleYaml,

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

//region Lifecycle

#[derive(serde::Deserialize, serde::Serialize)]
struct EntityLifecycleYaml {
    creation: EntityCreationYaml,

    #[serde(default)]
    deletion: EntityDeletionYaml,
}

impl convert::TryFrom<&EntityLifecycleYaml> for entity::EntityLifecycle {
    type Error = yaml::Error;

    fn try_from(x: &EntityLifecycleYaml) -> Result<Self, Self::Error> {
        Ok(entity::EntityLifecycle {
            creation: (&x.creation).try_into()?,
            deletion: (&x.deletion).try_into()?,
        })
    }
}

impl convert::TryFrom<EntityLifecycleYaml> for entity::EntityLifecycle {
    type Error = yaml::Error;

    fn try_from(x: EntityLifecycleYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

//endregion

//region Creation

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct EntityCreationYaml {
    #[serde(default)]
    documentation: String,

    allow_cross_process_callers: bool,
}

impl Default for EntityCreationYaml {
    fn default() -> EntityCreationYaml {
        EntityCreationYaml {
            documentation: "".to_owned(),
            allow_cross_process_callers: false,
        }
    }
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

//region Deletion

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EntityDeletionYaml {
    #[serde(default)]
    enabled: bool,

    #[serde(default)]
    notes: EntityDeletionNotesYaml,
}

impl Default for EntityDeletionYaml {
    fn default() -> EntityDeletionYaml {
        let x = entity::EntityDeletion::default();
        EntityDeletionYaml {
            enabled: x.enabled,
            notes: EntityDeletionNotesYaml::default(),
        }
    }
}

impl convert::TryFrom<&EntityDeletionYaml> for entity::EntityDeletion {
    type Error = yaml::Error;

    fn try_from(x: &EntityDeletionYaml) -> Result<Self, Self::Error> {
        Ok(entity::EntityDeletion {
            enabled: x.enabled,
            notes: (&x.notes).try_into()?,
        })
    }
}

impl convert::TryFrom<EntityDeletionYaml> for entity::EntityDeletion {
    type Error = error::Error;

    fn try_from(x: EntityDeletionYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EntityDeletionNotesYaml {
    #[serde(default)]
    enabled: bool,

    #[serde(default)]
    required: bool,
}

impl Default for EntityDeletionNotesYaml {
    fn default() -> EntityDeletionNotesYaml {
        let x = entity::EntityDeletionNotes::default();
        EntityDeletionNotesYaml {
            enabled: x.enabled,
            required: x.required,
        }
    }
}

impl convert::TryFrom<&EntityDeletionNotesYaml> for entity::EntityDeletionNotes {
    type Error = yaml::Error;

    fn try_from(x: &EntityDeletionNotesYaml) -> Result<Self, Self::Error> {
        Ok(entity::EntityDeletionNotes {
            enabled: x.enabled,
            required: x.required,
        })
    }
}

impl convert::TryFrom<EntityDeletionNotesYaml> for entity::EntityDeletionNotes {
    type Error = error::Error;

    fn try_from(x: EntityDeletionNotesYaml) -> Result<Self, Self::Error> {
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
