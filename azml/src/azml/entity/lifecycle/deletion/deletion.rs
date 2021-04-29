//

//TODO:
// - the type of deletion: recoverable or not
//TODO: deletion types/modes

use crate::azml::mixin;

#[derive(Clone, Debug)]
pub struct Deletion {
    // Wether the entity could be deleted.
    pub enabled: bool,
    pub notes: DeletionNotes,
}

impl Default for Deletion {
    fn default() -> Deletion {
        Deletion {
            enabled: false,
            notes: DeletionNotes::default(),
        }
    }
}

impl mixin::MixinDefinition for Deletion {}

#[derive(Clone, Debug)]
pub struct DeletionNotes {
    // Whether a deletion should include notes.
    pub enabled: bool,

    // Whether the notes is required.
    pub required: bool,
}

impl Default for DeletionNotes {
    fn default() -> DeletionNotes {
        DeletionNotes {
            enabled: false,
            required: false,
        }
    }
}
