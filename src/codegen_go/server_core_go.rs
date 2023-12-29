// Responsible for generating server core.

use std::{error, fs, io::Write};

use azml::azml::module;

use super::{template::render_template, BaseContext, GoCodeGenerator, ServiceContext};

impl GoCodeGenerator {
    pub fn generate_server_core_codes(
        &self,
        module_name: &String,
        _module_def: &module::ModuleDefinition,
        services: &Vec<ServiceContext>,
    ) -> Result<(), Box<dyn error::Error>> {
        let tpl_ctx = ServerCoreContext {
            base: self.render_base_context(),
            pkg_name: module_name.to_lowercase(),
            pkg_path: self.contract_package_identifier.to_owned(),
            services: services.to_owned(),
        };

        let type_name_snake = "server_core".to_owned();

        let header_tpl_bytes = include_bytes!("templates/server_core/server_core__header.gtmpl");
        let header_code = render_template(
            String::from_utf8_lossy(header_tpl_bytes).as_ref(),
            tpl_ctx.to_owned(),
        )?;

        let mut out_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!(
                "{}/{}__azgen.go",
                self.server_package_dir_base_name, type_name_snake
            ))?;
        out_file.write_all(header_code.as_bytes())?;

        Ok(())
    }
}

#[derive(Clone, Gtmpl)]
struct ServerCoreContext {
    base: BaseContext,
    pkg_name: String,
    pkg_path: String,
    services: Vec<ServiceContext>,
}
