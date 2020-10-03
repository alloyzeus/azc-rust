//

use std::{error, fs, io::Write};

use crate::codegen;

use azml::azml::{adjunct::adjunct, entity::entity, module, value_object::value_object};

#[macro_use]
mod render_macros;

mod adjunct_go;
mod entity_go;
mod template;
mod value_object_go;

pub struct GoCodeGenerator {
    // The target directory path
    pub base_dir: String,
    // Go module identifier. This is the one defined in the go.mod file.
    pub module_identifier: String,
    pub file_per_struct: bool,

    pub azlib_prefix: String,

    // AZCore is the fundamental part of the language
    pub azcore_import: String,
    pub azcore_pkg: String,
    // AZStd is a collection of well-thought, stable library
    // AZExt contains additional libraries which are generally optional
    // or they are in an experimental stage.
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

    fn id_size_from_space(id_space: i8) -> i8 {
        match id_space {
            d if d < 16 => 16,
            d if d < 32 => 32,
            d if d < 64 => 64,
            _ => -1, //TODO: error. we won't need this here. generators receive clean data.
        }
    }
}

impl codegen::CodeGenerator for GoCodeGenerator {
    fn generate_module_codes(
        &self,
        module_name: &String,
        module_def: &module::ModuleDefinition,
    ) -> Result<(), Box<dyn error::Error>> {
        let base_dir = self.base_dir.to_owned();
        let tpl_ctx = LibraryContext {
            base: self.render_base_context(),
            pkg_name: module_name.to_owned(),
        };
        fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
        let out_name = format!("{}/{}/AZLib.go", base_dir, module_name,);
        let out_tpl_bytes = include_bytes!("templates/az_lib.gtmpl");
        let out_code = gtmpl::template(
            String::from_utf8_lossy(out_tpl_bytes).as_ref(),
            tpl_ctx.to_owned(),
        )?;
        let mut out_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(out_name)?;
        out_file.write_all(out_code.as_bytes())?;
        drop(out_file);

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
