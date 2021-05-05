//

use std::convert::{self, TryInto};

use crate::azml::{entity::lifecycle::deletion::deletion, error, yaml};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DeletionYaml {
    #[serde(default)]
    enabled: bool,

    #[serde(default)]
    notes: DeletionNotesYaml,
}

impl Default for DeletionYaml {
    fn default() -> DeletionYaml {
        let x = deletion::Deletion::default();
        DeletionYaml {
            enabled: x.enabled,
            notes: DeletionNotesYaml::default(),
        }
    }
}

impl convert::TryFrom<&DeletionYaml> for deletion::Deletion {
    type Error = yaml::Error;

    fn try_from(x: &DeletionYaml) -> Result<Self, Self::Error> {
        Ok(deletion::Deletion {
            enabled: x.enabled,
            notes: (&x.notes).try_into()?,
        })
    }
}

impl convert::TryFrom<DeletionYaml> for deletion::Deletion {
    type Error = error::Error;

    fn try_from(x: DeletionYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DeletionNotesYaml {
    #[serde(default)]
    enabled: bool,

    #[serde(default)]
    required: bool,
}

impl Default for DeletionNotesYaml {
    fn default() -> DeletionNotesYaml {
        let x = deletion::DeletionNotes::default();
        DeletionNotesYaml {
            enabled: x.enabled,
            required: x.required,
        }
    }
}

impl convert::TryFrom<&DeletionNotesYaml> for deletion::DeletionNotes {
    type Error = yaml::Error;

    fn try_from(x: &DeletionNotesYaml) -> Result<Self, Self::Error> {
        Ok(deletion::DeletionNotes {
            enabled: x.enabled,
            required: x.required,
        })
    }
}

impl convert::TryFrom<DeletionNotesYaml> for deletion::DeletionNotes {
    type Error = error::Error;

    fn try_from(x: DeletionNotesYaml) -> Result<Self, Self::Error> {
        (&x).try_into()
    }
}