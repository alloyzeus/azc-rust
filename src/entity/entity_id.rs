//

#[derive(Debug)]
pub struct EntityId {
    pub parameters: Option<Box<dyn EntityIdDefinition>>,
}

pub trait EntityIdDefinition: mopa::Any + std::fmt::Debug {}

mopafy!(EntityIdDefinition);
