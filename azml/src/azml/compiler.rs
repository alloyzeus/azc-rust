//

use std::{collections::HashMap, convert::TryFrom, fs, io, path};

use crate::azml::{module, result, source_file, source_file_yaml, yaml};

#[derive(Debug)]
pub struct CompilationState {
    pub entry_module: String,
    pub modules: HashMap<String, module::ModuleDefinition>,
}

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn compile_file<P: AsRef<path::Path>>(&self, path: P) -> result::Result<CompilationState> {
        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);
        let sf: source_file_yaml::SourceFileYaml = yaml::from_reader(reader)?;
        let sf = source_file::SourceFile::try_from(sf)?;
        let mut modules = HashMap::new();
        modules.insert(
            sf.module.to_owned(),
            module::ModuleDefinition {
                symbols: sf.symbols.to_vec(),
            },
        );
        Ok(CompilationState {
            entry_module: sf.module.to_owned(),
            modules: modules,
        })
    }
}
