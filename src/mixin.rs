//

#[derive(Debug)]
pub struct Mixin {
    //TODO: non-optional
    pub parameters: Option<Box<dyn MixinDefinition>>,
}

pub trait MixinDefinition: mopa::Any + std::fmt::Debug {}

mopafy!(MixinDefinition);

#[derive(Debug)]
pub struct MixinField<T> {
    pub overridable: bool,
    pub value: T,
}
