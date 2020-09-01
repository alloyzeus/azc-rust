//

#[derive(Debug)]
pub struct Mixin {
    pub kind: String,

    //TODO: non-optional
    pub parameters: Option<Box<dyn std::any::Any>>,
}

#[derive(Debug)]
pub struct MixinField<T> {
    pub overridable: bool,
    pub value: T,
}
