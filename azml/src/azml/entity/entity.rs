//

use crate::azml::{abstract_, attribute, entity::entity_id_num, mixin, ref_key, symbol};

//region Entity

#[derive(Clone, Debug)]
pub struct Entity {
    pub id_num: entity_id_num::EntityIdNum,
    pub ref_key: ref_key::RefKey,
    pub implements: abstract_::AbstractImplementation,
    pub lifecycle: EntityLifecycle,
    pub mixins: Vec<mixin::Mixin>,
    pub service: Option<EntityService>,
    pub attributes: Vec<attribute::Attribute>,
}

impl symbol::SymbolDefinition for Entity {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        let a_syms = self
            .attributes
            .iter()
            .fold(Vec::<symbol::SymbolRef>::new(), |a, b| {
                a.into_iter()
                    .chain(b.collect_symbol_refs())
                    .collect::<Vec<_>>()
            });
        let id_syms = self.id_num.definition.collect_symbol_refs();
        a_syms.into_iter().chain(id_syms.into_iter()).collect()
    }
}

//endregion

//----

#[derive(Clone, Debug)]
pub struct EntityLifecycle {
    pub creation: EntityCreation,
    pub deletion: EntityDeletion,
}

// Special mixin.
//
// Creation is a special mixin which defines the rule for the creation
// of any instance.
#[derive(Clone, Debug)]
pub struct EntityCreation {
    pub documentation: String,
    pub allow_cross_process_callers: bool,
}

impl mixin::MixinDefinition for EntityCreation {}

//TODO: entity deletion types/modes

#[derive(Clone, Debug)]
pub struct EntityDeletion {
    // Wether entity could be deleted.
    pub enabled: bool,
    pub notes: EntityDeletionNotes,
}

impl Default for EntityDeletion {
    fn default() -> EntityDeletion {
        EntityDeletion {
            enabled: false,
            notes: EntityDeletionNotes::default(),
        }
    }
}

impl mixin::MixinDefinition for EntityDeletion {}

#[derive(Clone, Debug)]
pub struct EntityDeletionNotes {
    // Whether a deletion should include notes.
    pub enabled: bool,

    // Whether the notes is required.
    pub required: bool,
}

impl Default for EntityDeletionNotes {
    fn default() -> EntityDeletionNotes {
        EntityDeletionNotes {
            enabled: false,
            required: false,
        }
    }
}

//----

#[derive(Clone, Debug)]
pub struct EntityService {
    pub documentation: String,
    pub enabled: bool,
}
