//

#[derive(Clone, Debug)]
pub struct AbstractImplementation {
    pub kind: String,
    pub attributes: Vec<AbstractImplementationAttribute>,
}

#[derive(Clone, Debug)]
pub struct AbstractImplementationAttribute {
    pub identifier: String,
    pub kind: String,
}
