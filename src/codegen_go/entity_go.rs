//

use std::{error, fs, io::Write};

use crate::codegen_go::{BaseContext, GoCodeGenerator};

use azml::azml::{
    entity::{entity, entity_id_integer},
    symbol,
};

use crate::codegen_go::template::render_template;

impl GoCodeGenerator {
    pub fn generate_entity_codes(
        &self,
        module_name: &String,
        ent: &entity::Entity,
        symbol: &symbol::Symbol,
    ) -> Result<(), Box<dyn error::Error>> {
        let base_dir = &self.base_dir;
        let type_name = symbol.identifier.to_owned();
        let pkg_path = format!("{}/{}", self.module_identifier, module_name);
        let id_def = &ent.id.definition;

        if let Some(id_int) = id_def.downcast_ref::<entity_id_integer::EntityIdInteger>() {
            let id_size = Self::int_id_size_from_bits(id_int.bits);

            let id_type_name = format!("{}ID", type_name);
            let id_type_primitive = format!("int{}", id_size);
            let ref_key_type_name = format!("{}RefKey", type_name);
            let attrs_type_name = format!("{}Attributes", type_name);
            let event_interface_name = format!("{}Event", type_name);
            let service_name = format!("{}Service", type_name);
            let type_doc_lines: Vec<String> =
                symbol.documentation.lines().map(|x| x.to_owned()).collect();
            let attributes: Vec<EntityAttributeContext> = (&ent.attributes)
                .into_iter()
                .map(|x| EntityAttributeContext {
                    identifier: x.identifier.to_owned(),
                    type_name: x.kind.to_owned(),
                })
                .collect();

            let tpl_ctx = EntityContext {
                base: self.render_base_context(),
                pkg_name: module_name.to_lowercase(),
                pkg_path: pkg_path.to_owned(),
                type_name: type_name.to_owned(),
                type_doc_lines: type_doc_lines.clone(),
                id_type_name: id_type_name.to_owned(),
                id_type_primitive: id_type_primitive.to_owned(),
                ref_key_type_name: ref_key_type_name.to_owned(),
                attributes_type_name: attrs_type_name.to_owned(),
                attributes: attributes,
                event_interface_name: event_interface_name.to_owned(),
                service_name: service_name.to_owned(),
            };

            let header_tpl_bytes = include_bytes!("templates/entity__header.gtmpl");
            let header_code = render_template(
                String::from_utf8_lossy(header_tpl_bytes).as_ref(),
                tpl_ctx.to_owned(),
            )?;

            if !ent.attributes.is_empty() {
                println!("TODO: attributes for entity {}", type_name);
            }

            let mut out_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(format!("{}/{}/{}.go", base_dir, module_name, type_name))?;
            out_file.write_all(header_code.as_bytes())?;
            out_file.write_all(format!("\n// Entity {}.\n", type_name).as_bytes())?;
            if !type_doc_lines.is_empty() {
                out_file.write_all("//\n".as_bytes())?;
                for x in type_doc_lines {
                    out_file.write_all("// ".as_bytes())?;
                    out_file.write_all(x.as_bytes())?;
                    out_file.write_all("\n".as_bytes())?;
                }
            }
            render_file_append!(out_file, "templates/entity_id.gtmpl", tpl_ctx);
            render_file_append!(out_file, "templates/entity_ref_key.gtmpl", tpl_ctx);
            render_file_append!(out_file, "templates/entity_attributes.gtmpl", tpl_ctx);
            render_file_append!(out_file, "templates/entity_event.gtmpl", tpl_ctx);
            render_file_append!(out_file, "templates/entity_service.gtmpl", tpl_ctx);
            render_file_append!(out_file, "templates/entity_service_base.gtmpl", tpl_ctx);

            // ServiceClient
            render_file!(
                format!("{}/{}/client", base_dir, module_name,),
                format!("{}ClientBase", service_name),
                "templates/entity_service_client_base.gtmpl",
                tpl_ctx,
                ""
            );

            // ServiceServer
            render_file!(
                format!("{}/{}server", base_dir, module_name,),
                format!("{}Server", service_name),
                "templates/entity_service_server.gtmpl",
                tpl_ctx,
                ""
            );

            Ok(())
        } else {
            Err(Box::new(azml::azml::Error::Msg(
                "Unsupported ID type".to_owned(),
            )))
        }
    }
}

#[derive(Clone, Gtmpl)]
struct EntityContext {
    base: BaseContext,
    pkg_name: String,
    pkg_path: String,
    type_name: String,
    type_doc_lines: Vec<String>,
    id_type_name: String,
    id_type_primitive: String,
    ref_key_type_name: String,
    attributes_type_name: String,
    attributes: Vec<EntityAttributeContext>,
    event_interface_name: String,
    service_name: String,
}

#[derive(Clone, Gtmpl)]
struct EntityAttributeContext {
    identifier: String,
    type_name: String,
}
