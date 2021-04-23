//

use std::{convert, convert::TryInto};

use crate::azfl::deletion::deletion;
use crate::azml::error;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DeletionYaml {
    #[serde(default)]
    notes: DeletionNotesYaml,
}

impl convert::TryFrom<DeletionYaml> for deletion::Deletion {
    type Error = error::Error;

    fn try_from(x: DeletionYaml) -> Result<Self, Self::Error> {
        Ok(deletion::Deletion {
            notes: x.notes.try_into()?,
        })
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

impl convert::TryFrom<DeletionNotesYaml> for deletion::DeletionNotes {
    type Error = error::Error;

    fn try_from(x: DeletionNotesYaml) -> Result<Self, Self::Error> {
        Ok(deletion::DeletionNotes {
            enabled: x.enabled,
            required: x.required,
        })
    }
}
