//

#[derive(Clone, Debug)]
pub struct Mixin {
    //TODO: non-optional
    pub definition: Option<Box<dyn MixinDefinition>>,
}

pub trait MixinDefinition: mopa::Any + MixinDefinitionClone + std::fmt::Debug {}

mopafy!(MixinDefinition);

pub trait MixinDefinitionClone {
    fn clone_box(&self) -> Box<dyn MixinDefinition>;
}

impl<T> MixinDefinitionClone for T
where
    T: MixinDefinition + Clone,
{
    fn clone_box(&self) -> Box<dyn MixinDefinition> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn MixinDefinition> {
    fn clone(&self) -> Box<dyn MixinDefinition> {
        self.clone_box()
    }
}

#[derive(Clone, Debug)]
pub struct MixinField<T> {
    pub overridable: bool,
    pub value: T,
}
