//

use std::fmt;

//region GeneratorOptions

pub trait GeneratorOptions: mopa::Any + GeneratorOptionsClone + fmt::Debug {}

mopafy!(GeneratorOptions);

pub trait GeneratorOptionsClone {
    fn clone_boxed_generator_option(&self) -> Box<dyn GeneratorOptions>;
}

impl<T> GeneratorOptionsClone for T
where
    T: GeneratorOptions + Clone,
{
    fn clone_boxed_generator_option(&self) -> Box<dyn GeneratorOptions> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn GeneratorOptions> {
    fn clone(&self) -> Box<dyn GeneratorOptions> {
        self.clone_boxed_generator_option()
    }
}

//endregion
