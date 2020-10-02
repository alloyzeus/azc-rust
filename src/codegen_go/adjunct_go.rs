//

use std::{error, fs, io::Write};

use crate::codegen_go::{BaseContext, GoCodeGenerator};

use azml::azml::{
    adjunct::{adjunct, adjunct_entity},
    symbol,
};

impl GoCodeGenerator {
    pub fn generate_adjunct_entity_codes(
        &self,
        module_name: &String,
        adj_ent: &adjunct_entity::AdjunctEntity,
        symbol: &symbol::Symbol,
        hosts: &Vec<adjunct::AdjunctHost>,
    ) -> Result<(), Box<dyn error::Error>> {
        let base_dir = &self.base_dir;
        let type_name = symbol.identifier.to_owned();
        let pkg_path = format!("{}/{}", self.module_identifier, module_name);
        let hosts_names = hosts
            .into_iter()
            .map(|x| x.name.to_owned())
            .collect::<Vec<String>>();
        //TODO: if the adjunct is globally addressable, i.e., an instance's
        // ID is unique system-wide, it must not derive its hosts' name
        // by default.
        // And also, the RefKey is just a typedef of ID.
        let global_scope = adjunct_entity::AdjunctEntityScope::Global == adj_ent.scope;
        let base_type_name = if global_scope {
            "".to_owned()
        } else {
            hosts_names.join("")
        };

        let type_name = format!("{}{}", base_type_name, type_name);
        let id_type_name = format!("{}ID", type_name);
        let id_type_primitive = format!("int{}", 64); //TODO: de-hardcode
        let ref_key_type_name = format!("{}RefKey", type_name);
        let attrs_type_name = format!("{}Attributes", type_name);
        let service_name = format!("{}Service", type_name);
        let type_doc_lines: Vec<String> =
            symbol.documentation.lines().map(|x| x.to_owned()).collect();

        let tpl_ctx = AdjunctEntityContext {
            base: self.render_base_context(),
            pkg_name: module_name.to_lowercase(),
            pkg_path: pkg_path.to_owned(),
            type_name: type_name.to_owned(),
            id_type_name: id_type_name.to_owned(),
            id_type_primitive: id_type_primitive.to_owned(),
            ref_key_type_name: ref_key_type_name.to_owned(),
            attributes_type_name: attrs_type_name.to_owned(),
            service_name: service_name.to_owned(),
            hosts: hosts_names.clone(),
            global_scope: global_scope,
        };

        let header_tpl_bytes = include_bytes!("templates/adjunct_entity__header.gtmpl");
        let header_code = gtmpl::template(
            String::from_utf8_lossy(header_tpl_bytes).as_ref(),
            tpl_ctx.to_owned(),
        )?;

        if self.file_per_struct {
            // ID
            render_file!(
                format!("{}/{}", base_dir, module_name,),
                id_type_name,
                "templates/adjunct_entity_id.gtmpl",
                tpl_ctx,
                header_code
            );

            // RefKey
            render_file!(
                format!("{}/{}", base_dir, module_name,),
                ref_key_type_name,
                "templates/adjunct_entity_ref_key.gtmpl",
                tpl_ctx,
                header_code
            );

            // Attributes
            render_file!(
                format!("{}/{}", base_dir, module_name,),
                attrs_type_name,
                "templates/adjunct_entity_attributes.gtmpl",
                tpl_ctx,
                header_code
            );

            // Service
            render_file!(
                format!("{}/{}", base_dir, module_name,),
                service_name,
                "templates/adjunct_entity_service.gtmpl",
                tpl_ctx,
                header_code
            );
        } else {
            let mut out_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(format!("{}/{}/{}.go", base_dir, module_name, type_name))?;

            out_file.write_all(header_code.as_bytes())?;
            out_file.write_all(
                format!(
                    "\n// Adjunct-entity {} of {}.\n",
                    type_name,
                    hosts_names.join(", ")
                )
                .as_bytes(),
            )?;
            if !type_doc_lines.is_empty() {
                out_file.write_all("//\n".as_bytes())?;
                for x in type_doc_lines {
                    out_file.write_all("// ".as_bytes())?;
                    out_file.write_all(x.as_bytes())?;
                    out_file.write_all("\n".as_bytes())?;
                }
            }
            render_file_append!(out_file, "templates/adjunct_entity_id.gtmpl", tpl_ctx);
            render_file_append!(out_file, "templates/adjunct_entity_ref_key.gtmpl", tpl_ctx);
            //render_file_append!(out_file, "templates/adjunct_entity_event.gtmpl", tpl_ctx);
            render_file_append!(
                out_file,
                "templates/adjunct_entity_attributes.gtmpl",
                tpl_ctx
            );
            render_file_append!(out_file, "templates/adjunct_entity_service.gtmpl", tpl_ctx);
        }

        Ok(())
    }
}

#[derive(Clone, Gtmpl)]
struct AdjunctEntityContext {
    base: BaseContext,
    pkg_name: String,
    pkg_path: String,
    type_name: String,
    id_type_name: String,
    id_type_primitive: String,
    ref_key_type_name: String,
    attributes_type_name: String,
    service_name: String,
    hosts: Vec<String>,
    global_scope: bool,
}
