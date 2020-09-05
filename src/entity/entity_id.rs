//

#[derive(Debug)]
pub struct EntityIdDefinition {
    pub kind: String,
    pub parameters: Option<Box<dyn EntityIdParameters>>,
}

pub trait EntityIdParameters: mopa::Any + std::fmt::Debug {}

mopafy!(EntityIdParameters);
