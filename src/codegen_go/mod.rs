//

use std::{collections::HashMap, error, fs, io::Write};

use crate::codegen;

use azml::azml::{
    adjunct::adjunct,
    compiler,
    entity::{abstract_, entity, root_entity},
    generator_go::GeneratorGoPackagesOptions,
    module, symbol,
    value_object::value_object,
};

use template::render_template;

#[macro_use]
mod render_macros;

mod adjunct_entity_go;
mod adjunct_go;
mod adjunct_prime_go;
mod adjunct_value_go;
mod attribute_go;
mod entity_go;
mod id_num_go;
mod ref_key_go;
mod server_core_go;
mod symbol_go;
mod template;
mod value_object_go;

pub struct GoCodeGenerator {
    // The target directory path
    pub base_dir: String,
    pub base_pkg: String,

    // Go module identifier. This is the one defined in the go.mod file.
    pub module_identifier: GeneratorGoPackagesOptions,

    // A flag to make the generator generates server package(s) and application(s).
    // Curently unused.
    pub generate_servers: bool,
    // A flag to render every Go struct to its own file. Currently unused.
    pub file_per_struct: bool,

    pub package_urls: HashMap<String, String>,

    pub iam_pkg: String,
    pub azlib_prefix: String,

    // AZCore is the fundamental part of the language
    pub azcore_import: String,
    pub azcore_pkg: String,
    pub azid_import: String,
    pub azid_pkg: String,
    pub azob_import: String,
    pub azob_pkg: String,
    pub azerrs_import: String,
    pub azerrs_pkg: String,
    // AZStd is a collection of well-thought, stable library
    // AZExt contains additional libraries which are generally optional
    // or they are in an experimental stage.
    pub compilation_state: Option<compiler::CompilationState>,
    // Full package identifier including module and entry package name
    pub contract_package_identifier: String,
    pub contract_package_dir_base_name: String,
    pub server_package_identifier: String,
    pub server_package_dir_base_name: String,
    pub client_package_identifier: String,
    pub client_package_dir_base_name: String,

    pub service_op_call_context_type_name: String,
}

impl GoCodeGenerator {
    fn render_base_context(&self) -> BaseContext {
        BaseContext {
            mod_name: PackagesContext {
                contract: self.module_identifier.contract.to_owned(),
                server: self.module_identifier.server.to_owned(),
                client: self.module_identifier.client.to_owned(),
            },
            iam_pkg: self.iam_pkg.to_owned(),
            azlib_prefix: self.azlib_prefix.to_owned(),
            azcore_import: self.azcore_import.to_owned(),
            azcore_pkg: self.azcore_pkg.to_owned(),
            azcore_version: "AZCorePackageIsVersion1".to_owned(),
            azid_import: self.azid_import.to_owned(),
            azid_pkg: self.azid_pkg.to_owned(),
            azob_import: self.azob_import.to_owned(),
            azob_pkg: self.azob_pkg.to_owned(),
            azerrs_import: self.azerrs_import.to_owned(),
            azerrs_pkg: self.azerrs_pkg.to_owned(),
            //TODO: these below should be resolved by the compiler
            terminal: TerminalContext {
                pg_type: "bigint".to_owned(),
            },
            user: UserContext {
                pg_type: "bigint".to_owned(),
            },
            service_op_call_context_type_name: self.service_op_call_context_type_name.to_owned(),
        }
    }

    //TODO: contextual and scoped resolver
    fn resolve_import(&self, pkg: &String) -> String {
        match self.package_urls.get(pkg) {
            Some(s) => s.to_owned(),
            _ => "???".to_owned(),
        }
    }

    fn lookup_entity(&self, entity_ref: symbol::SymbolRef) -> Option<Box<&dyn entity::Entity>> {
        match &self.compilation_state {
            Some(compilation_state) => compilation_state.lookup_entity(entity_ref),
            _ => None,
        }
    }

    fn lookup_abstract(&self, sym_ref: symbol::SymbolRef) -> Option<&abstract_::Abstract> {
        match &self.compilation_state {
            Some(compilation_state) => compilation_state.lookup_abstract(sym_ref),
            _ => None,
        }
    }

    fn generate_module_codes(
        &self,
        module_name: &String,
        module_def: &module::ModuleDefinition,
    ) -> Result<(), Box<dyn error::Error>> {
        let contract_target_dir = &self.contract_package_dir_base_name;
        fs::create_dir_all(contract_target_dir)?;

        let server_target_dir = &self.server_package_dir_base_name;
        fs::create_dir_all(server_target_dir)?;

        let client_target_dir = &self.client_package_dir_base_name;
        fs::create_dir_all(client_target_dir)?;

        // let filename_prefix = self.azlib_prefix.to_lowercase();

        // render_file!(
        //     contract_target_dir,
        //     format!("{}_service", filename_prefix),
        //     "templates/azlib_service.gtmpl",
        //     tpl_ctx,
        //     ""
        // );

        // render_file!(
        //     contract_target_dir,
        //     format!("{}_entity", filename_prefix),
        //     "templates/azlib_entity.gtmpl",
        //     tpl_ctx,
        //     ""
        // );

        // render_file!(
        //     client_target_dir,
        //     format!("{}_entity_service_client", filename_prefix),
        //     "templates/azlib_entity_service_client.gtmpl",
        //     tpl_ctx,
        //     ""
        // );

        // render_file!(
        //     server_target_dir,
        //     format!("{}_entity_service_server", filename_prefix),
        //     "templates/azlib_entity_service_server.gtmpl",
        //     tpl_ctx,
        //     ""
        // );

        // render_file!(
        //     contract_target_dir,
        //     format!("{}_adjunct", filename_prefix),
        //     "templates/azlib_adjunct.gtmpl",
        //     tpl_ctx,
        //     ""
        // );

        let mut services: Vec<ServiceContext> = vec![];

        for symbol in &module_def.symbols {
            let params = &symbol.definition;
            if let Some(ent) = params.downcast_ref::<root_entity::RootEntity>() {
                self.generate_root_entity_codes(module_name, ent, &symbol)?;
                services.extend(self.root_entity_server_fields(module_name, ent, &symbol)?);
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

        let tpl_ctx = LibraryContext {
            base: self.render_base_context(),
            pkg_name: module_name.to_owned(),
            services: services.to_owned(),
        };

        render_file!(
            contract_target_dir,
            "service__azgen",
            "templates/service/service.gtmpl",
            tpl_ctx,
            ""
        );

        self.generate_server_core_codes(module_name, module_def, &services)?;

        Ok(())
    }
}

impl codegen::CodeGenerator for GoCodeGenerator {
    fn generate_codes(
        &mut self,
        compilation_state: &compiler::CompilationState,
    ) -> Result<(), Box<dyn error::Error>> {
        self.compilation_state = Some(compilation_state.clone());
        self.contract_package_identifier = if self.base_pkg.is_empty() {
            format!(
                "{}/{}",
                self.module_identifier.contract, compilation_state.entry_module
            )
        } else {
            format!(
                "{}/{}/{}",
                self.module_identifier.contract, self.base_pkg, compilation_state.entry_module
            )
        };
        self.contract_package_dir_base_name =
            format!("{}/{}", self.base_dir, self.contract_package_identifier);
        self.server_package_identifier = if self.base_pkg.is_empty() {
            format!(
                "{}/{}server",
                self.module_identifier.server, compilation_state.entry_module
            )
        } else {
            format!(
                "{}/{}/{}server",
                self.module_identifier.server, self.base_pkg, compilation_state.entry_module
            )
        };
        self.server_package_dir_base_name =
            format!("{}/{}", self.base_dir, self.server_package_identifier);
        self.client_package_identifier = if self.base_pkg.is_empty() {
            format!(
                "{}/{}",
                self.module_identifier.client, compilation_state.entry_module
            )
        } else {
            format!(
                "{}/{}/{}",
                self.module_identifier.client, self.base_pkg, compilation_state.entry_module
            )
        };
        self.client_package_dir_base_name =
            format!("{}/{}", self.base_dir, self.client_package_identifier);
        let entry_module = compilation_state
            .modules
            .get(&compilation_state.entry_module);
        match entry_module {
            Some(entry_module) => self.generate_module_codes(
                &compilation_state.entry_module,
                &module::ModuleDefinition {
                    symbols: entry_module.symbols.to_vec(),
                    generator_options: entry_module.generator_options.clone(),
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
    mod_name: PackagesContext,
    iam_pkg: String,
    azlib_prefix: String,
    azcore_import: String,
    azcore_pkg: String,
    azcore_version: String,
    azid_import: String,
    azid_pkg: String,
    azob_import: String,
    azob_pkg: String,
    azerrs_import: String,
    azerrs_pkg: String,
    terminal: TerminalContext,
    user: UserContext,
    service_op_call_context_type_name: String,
}

#[derive(Clone, Gtmpl)]
struct PackagesContext {
    contract: String,
    server: String,
    client: String,
}

#[derive(Clone, Gtmpl)]
struct LibraryContext {
    base: BaseContext,
    pkg_name: String,
    services: Vec<ServiceContext>,
}

#[derive(Clone, Gtmpl)]
struct ImportContext {
    alias: String,
    url: String,
}

#[derive(Clone, Gtmpl)]
struct TerminalContext {
    pg_type: String,
}

#[derive(Clone, Gtmpl)]
struct UserContext {
    pg_type: String,
}

#[derive(Clone, Gtmpl)]
pub struct ServiceContext {
    pub field_name: String,
    pub type_name: String,
    pub server_name: String,
}
