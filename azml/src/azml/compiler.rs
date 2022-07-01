//

use std::{collections::HashMap, convert::TryFrom, fs, io, path};

use super::{
    adjunct::adjunct_entity,
    entity::{abstract_, entity, root_entity},
    module, result, source_file, source_file_yaml, symbol, yaml,
};

#[derive(Clone, Debug)]
pub struct CompilationState {
    pub entry_module: String,
    pub modules: HashMap<String, module::ModuleDefinition>,
}

impl CompilationState {
    pub fn lookup_entity(&self, entity_ref: symbol::SymbolRef) -> Option<Box<&dyn entity::Entity>> {
        let module = self.modules.get(&entity_ref.package_identifier);
        match module {
            Some(module) => {
                let sym = module
                    .symbols
                    .iter()
                    .find(|&x| x.identifier == entity_ref.symbol_name);
                match sym {
                    Some(sym) => {
                        if let Some(e) = sym.definition.downcast_ref::<root_entity::RootEntity>() {
                            return Some(Box::new(e));
                        }
                        if let Some(e) = sym
                            .definition
                            .downcast_ref::<adjunct_entity::AdjunctEntity>()
                        {
                            return Some(Box::new(e));
                        }
                        if let Some(e) = sym.definition.downcast_ref::<abstract_::Abstract>() {
                            return Some(Box::new(e));
                        }
                        None
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
    pub fn lookup_abstract(&self, entity_ref: symbol::SymbolRef) -> Option<&abstract_::Abstract> {
        let module = self.modules.get(&entity_ref.package_identifier);
        match module {
            Some(module) => {
                let sym = module
                    .symbols
                    .iter()
                    .find(|&x| x.identifier == entity_ref.symbol_name);
                match sym {
                    Some(sym) => {
                        if let Some(e) = sym.definition.downcast_ref::<abstract_::Abstract>() {
                            Some(e)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn compile(&self) {
        //TODO: after we loaded all the sources, we do the real compilation
        // here.
        // We enforce rules, and load referenced sources.
        //
        // Rules to be enforced (incomplete):
        //
        // - ensure referenced symbols are valid and resolvable
        // - resolve implementations of abstract system objects and ensure that each has only one implementation
        // - ensure id bits
        // - ensure bitfield boundaries
        // - generate enums for bitfields
        // - ensure that adjunct hosts are entities
    }
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
            "_azsys".to_owned(),
            module::ModuleDefinition {
                symbols: vec![
                    symbol::Symbol {
                        identifier: "User".to_owned(),
                        documentation: "".to_owned(),
                        definition: Box::new(abstract_::Abstract {
                            documentation: "".to_owned(),
                            singular: true,
                            attributes: Vec::new(),
                        }),
                    },
                    symbol::Symbol {
                        identifier: "Terminal".to_owned(),
                        documentation: "".to_owned(),
                        definition: Box::new(abstract_::Abstract {
                            documentation: "".to_owned(),
                            singular: true,
                            attributes: Vec::new(),
                        }),
                    },
                    symbol::Symbol {
                        identifier: "Session".to_owned(),
                        documentation: "".to_owned(),
                        definition: Box::new(abstract_::Abstract {
                            documentation: "".to_owned(),
                            singular: true,
                            attributes: Vec::new(),
                        }),
                    },
                ],
                generator_options: HashMap::new(),
            },
        );

        modules.insert(
            sf.module.to_owned(),
            module::ModuleDefinition {
                symbols: sf.symbols.to_vec(),
                generator_options: sf.generator_options.clone(),
            },
        );

        let state = CompilationState {
            entry_module: sf.module.to_owned(),
            modules: modules,
        };

        state.compile();

        Ok(state)
    }
}
