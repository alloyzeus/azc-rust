//

use crate::azml::{
    attribute,
    entity::{abstract_, entity, id::id, lifecycle},
    mixin, symbol,
};

//region RootEntity

#[derive(Clone, Debug)]
pub struct RootEntity {
    pub id: id::Id,
    pub implements: Vec<abstract_::AbstractImplementation>,
    pub lifecycle: lifecycle::Lifecycle,
    pub mixins: Vec<mixin::Mixin>,
    pub service: Option<entity::EntityService>,
    pub attributes: Vec<attribute::Attribute>,
}

impl entity::Entity for RootEntity {}

impl symbol::SymbolDefinition for RootEntity {
    fn collect_symbol_refs(&self) -> Vec<symbol::SymbolRef> {
        let a_syms = self
            .attributes
            .iter()
            .fold(Vec::<symbol::SymbolRef>::new(), |a, b| {
                a.into_iter()
                    .chain(b.collect_symbol_refs())
                    .collect::<Vec<_>>()
            });
        let id_syms = self.id.num.definition.collect_symbol_refs();
        a_syms
            .iter()
            .chain(id_syms.iter())
            .map(|x| x.clone())
            .collect()
    }
}

//endregion
