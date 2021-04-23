//

//TODO:
// - the type of deletion: recoverable or not

use crate::azml::mixin;

#[derive(Clone, Debug)]
pub struct Deletion {
    pub notes: DeletionNotes,
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
