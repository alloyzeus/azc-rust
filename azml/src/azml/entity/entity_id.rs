//

#[derive(Clone, Debug)]
pub struct EntityId {
    pub parameters: Box<dyn EntityIdDefinition>,
}

pub trait EntityIdDefinition: mopa::Any + EntityIdDefinitionClone + std::fmt::Debug {}

mopafy!(EntityIdDefinition);

pub trait EntityIdDefinitionClone {
    fn clone_box(&self) -> Box<dyn EntityIdDefinition>;
}

impl<T> EntityIdDefinitionClone for T
where
    T: EntityIdDefinition + Clone,
{
    fn clone_box(&self) -> Box<dyn EntityIdDefinition> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn EntityIdDefinition> {
    fn clone(&self) -> Box<dyn EntityIdDefinition> {
        self.clone_box()
    }
}
