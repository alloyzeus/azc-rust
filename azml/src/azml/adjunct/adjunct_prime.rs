//

use crate::azml::{adjunct::adjunct, entity::abstract_, symbol};

// NOTE: the terminology is not final.
//
// An AdjunctPrime is a type of adjunct which have the characteristic
// of an entity, similar to AdjunctEntity, but it doesn't have its own
// identity. An AdjunctPrime instance gets the identifier from its host(s).
//
// It is implied that for an AdjunctPrime, that there will be only one
// instance of adjunct for a host or a set of host entity (one-to-one
// relationship). The cardinality attribute where the adjunct is defined
// is ignored.
//
// As an example, GroupChat needs to be entity-ish as Messages are adjunct
// of respective instance of GroupChat. But the GroupChat itself doesn't
// need to be full-blown adjunct-entity as for each Group, there will be only
// one GroupChat. A GroupChat doesn't need to have an identifier other than
// as required by its adjuncts; we should be able to use Group's identifier
// to resolve its GroupChat.
//
//TODO:
// - finalize the terminology
// - identifier prefix that is different from host entity

#[derive(Clone, Debug)]
pub struct AdjunctPrime {
    pub documentation: String,
    pub implements: Vec<abstract_::AbstractImplementation>,
    pub identity: AdjunctPrimeIdentity,
}

impl adjunct::AdjuctDefinition for AdjunctPrime {}

impl symbol::SymbolDefinition for AdjunctPrime {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        Vec::new()
    }
}

#[derive(Clone, Debug)]
pub struct AdjunctPrimeIdentity {
    pub prefix: String,
}
