//

use azml::azml::module;

//TODO: custom error type
pub trait CodeGenerator {
    fn generate_module_codes(
        &self,
        module_name: &String,
        module_def: &module::ModuleDefinition,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
