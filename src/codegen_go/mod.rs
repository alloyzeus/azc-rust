//

use std::{collections::HashMap, error, fs, io::Write};

use crate::codegen;

use crate::codegen_go::template::render_template;

use azml::azml::{adjunct::adjunct, compiler, entity::entity, module, value_object::value_object};

#[macro_use]
mod render_macros;

mod adjunct_go;
mod attribute_go;
mod eid_go;
mod entity_go;
mod ref_key_go;
mod symbol_go;
mod template;
mod value_object_go;

pub struct GoCodeGenerator {
    // The target directory path
    pub base_dir: String,
    pub base_pkg: String,

    // Go module identifier. This is the one defined in the go.mod file.
    pub module_identifier: String,

    // A flag to make the generator generates server package(s) and application(s).
    // Curently unused.
    pub generate_servers: bool,
    // A flag to render every Go struct to its own file. Currently unused.
    pub file_per_struct: bool,

    pub package_urls: HashMap<String, String>,

    pub azlib_prefix: String,

    // AZCore is the fundamental part of the language
    pub azcore_import: String,
    pub azcore_pkg: String,
    // AZStd is a collection of well-thought, stable library
    // AZExt contains additional libraries which are generally optional
    // or they are in an experimental stage.
    pub compilation_state: Option<compiler::CompilationState>,
    // Full package identifier including module and entry package name
    pub package_identifier: String,
    pub package_dir_base_name: String,
}

impl GoCodeGenerator {
    fn render_base_context(&self) -> BaseContext {
        BaseContext {
            mod_name: self.module_identifier.to_owned(),
            azlib_prefix: self.azlib_prefix.to_owned(),
            azcore_import: self.azcore_import.to_owned(),
            azcore_pkg: self.azcore_pkg.to_owned(),
            azcore_version: "AZCorePackageIsVersion1".to_owned(),
        }
    }

    //TODO: contextual and scoped resolver
    fn resolve_import(&self, pkg: &String) -> String {
        match self.package_urls.get(pkg) {
            Some(s) => s.to_owned(),
            _ => "???".to_owned(),
        }
    }

    // fn get_entity(&self, module: String, entity_name: String) -> Option<&entity::Entity> {
    //     match &self.compilation_state {
    //         Some(compilation_state) => compilation_state.get_entity(module, entity_name),
    //         _ => None,
    //     }
    // }
}

impl GoCodeGenerator {
    fn generate_module_codes(
        &self,
        module_name: &String,
        module_def: &module::ModuleDefinition,
    ) -> Result<(), Box<dyn error::Error>> {
        let tpl_ctx = LibraryContext {
            base: self.render_base_context(),
            pkg_name: module_name.to_owned(),
        };

        let target_dir = &self.package_dir_base_name;
        let filename_prefix = self.azlib_prefix.to_lowercase();

        render_file!(
            target_dir,
            format!("{}_service", filename_prefix),
            "templates/azlib_service.gtmpl",
            tpl_ctx,
            ""
        );

        render_file!(
            target_dir,
            format!("{}_entity", filename_prefix),
            "templates/azlib_entity.gtmpl",
            tpl_ctx,
            ""
        );

        render_file!(
            target_dir,
            format!("{}_entity_service_client", filename_prefix),
            "templates/azlib_entity_service_client.gtmpl",
            tpl_ctx,
            ""
        );

        render_file!(
            target_dir,
            format!("{}_entity_service_server", filename_prefix),
            "templates/azlib_entity_service_server.gtmpl",
            tpl_ctx,
            ""
        );

        render_file!(
            target_dir,
            format!("{}_adjunct", filename_prefix),
            "templates/azlib_adjunct.gtmpl",
            tpl_ctx,
            ""
        );

        for symbol in &module_def.symbols {
            let params = &symbol.definition;
            if let Some(ent) = params.downcast_ref::<entity::Entity>() {
                self.generate_entity_codes(module_name, ent, &symbol)?;
                continue;
            }
            if let Some(adj) = params.downcast_ref::<adjunct::Adjunct>() {
                self.generate_adjunct_codes(module_name, adj, &symbol)?;
                continue;
            }
            if let Some(vo) = params.downcast_ref::<value_object::ValueObject>() {
                self.generate_value_object_codes(module_name, &symbol, vo)?;
                continue;
            }
        }
        Ok(())
    }
}

impl codegen::CodeGenerator for GoCodeGenerator {
    fn generate_codes(
        &mut self,
        compilation_state: &compiler::CompilationState,
    ) -> Result<(), Box<dyn error::Error>> {
        self.compilation_state = Some(compilation_state.clone());
        self.package_identifier = if self.base_pkg.is_empty() {
            format!(
                "{}/{}",
                self.module_identifier, compilation_state.entry_module
            )
        } else {
            format!(
                "{}/{}/{}",
                self.module_identifier, self.base_pkg, compilation_state.entry_module
            )
        };
        self.package_dir_base_name = format!("{}/{}", self.base_dir, self.package_identifier);
        let entry_module = compilation_state
            .modules
            .get(&compilation_state.entry_module);
        match entry_module {
            Some(entry_module) => self.generate_module_codes(
                &compilation_state.entry_module,
                &module::ModuleDefinition {
                    symbols: entry_module.symbols.to_vec(),
                },
            ),
            _ => Err(Box::new(azml::azml::Error::Msg(
                "invalid compilation state".to_owned(),
            ))),
        }
    }
}

#[derive(Clone, Gtmpl)]
struct BaseContext {
    mod_name: String,
    azlib_prefix: String,
    azcore_import: String,
    azcore_pkg: String,
    azcore_version: String,
}

#[derive(Clone, Gtmpl)]
struct LibraryContext {
    base: BaseContext,
    pkg_name: String,
}

#[derive(Clone, Gtmpl)]
struct ImportContext {
    alias: String,
    url: String,
}
