//

use azml::azml::compiler;

//TODO: custom error type
pub trait CodeGenerator {
    fn generate_codes(
        &mut self,
        compilation_state: &compiler::CompilationState,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

// pub struct CodeGenerationConfig {

// }
