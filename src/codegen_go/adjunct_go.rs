//

use std::{error, fs, io::Write};

use crate::codegen_go::{attribute_go::AttributeContext, BaseContext, GoCodeGenerator};

use azml::azml::{
    adjunct::{adjunct, adjunct_entity, adjunct_value_object},
    symbol,
};

use crate::codegen_go::template::render_template;

impl GoCodeGenerator {
    pub fn generate_adjunct_codes(
        &self,
        module_name: &String,
        adj: &adjunct::Adjunct,
        sym: &symbol::Symbol,
    ) -> Result<(), Box<dyn error::Error>> {
        if let Some(adj_ent) = adj
            .definition
            .downcast_ref::<adjunct_entity::AdjunctEntity>()
        {
            self.generate_adjunct_entity_codes(module_name, adj_ent, &adj, &sym)?;
            Ok(())
        } else if let Some(adj_vo) = adj
            .definition
            .downcast_ref::<adjunct_value_object::AdjunctValueObject>()
        {
            self.generate_adjunct_value_object_codes(module_name, adj_vo, &adj, &sym)?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn generate_adjunct_entity_codes(
        &self,
        module_name: &String,
        adj_ent: &adjunct_entity::AdjunctEntity,
        adj: &adjunct::Adjunct,
        sym: &symbol::Symbol,
    ) -> Result<(), Box<dyn error::Error>> {
        let base_dir = &self.base_dir;
        let type_name = sym.identifier.to_owned();
        let pkg_path = format!("{}/{}", self.module_identifier, module_name);
        let hosts_names = (&adj.hosts)
            .into_iter()
            .map(|x| x.name.to_owned())
            .collect::<Vec<String>>();
        // If the adjunct is globally addressable, i.e., an instance's
        // ID is unique system-wide, it must not derive its hosts' name
        // by default.
        // And also, the RefKey is just a typedef of ID.
        let global_scope = adjunct_entity::AdjunctEntityScope::Global == adj_ent.scope;
        let base_type_name = if adj.name_is_prepared || global_scope {
            "".to_owned()
        } else {
            hosts_names.join("")
        };

        let type_name = format!("{}{}", base_type_name, type_name);
        let id_def = &adj_ent.id.definition;

        if let Some(id_int) = id_def.downcast_ref::<adjunct_entity::AdjunctEntityIdInteger>() {
            let id_size = Self::int_id_size_from_bits(id_int.bits);

            let id_type_name = format!("{}ID", type_name);
            let id_type_primitive = format!("int{}", id_size);
            let ref_key_type_name = format!("{}RefKey", type_name);
            let attrs_type_name = format!("{}Attributes", type_name);
            let attributes: Vec<AttributeContext> = (&adj_ent.attributes)
                .into_iter()
                .map(|x| AttributeContext {
                    identifier: x.identifier.to_owned(),
                    type_name: (&x.kind).into(),
                    kind: (&x.kind).into(),
                })
                .collect();
            let service_name = format!("{}Service", type_name);
            let type_doc_lines: Vec<String> =
                sym.documentation.lines().map(|x| x.to_owned()).collect();

            let tpl_ctx = AdjunctEntityContext {
                base: self.render_base_context(),
                pkg_name: module_name.to_lowercase(),
                pkg_path: pkg_path.to_owned(),
                type_name: type_name.to_owned(),
                id_type_name: id_type_name.to_owned(),
                id_type_primitive: id_type_primitive.to_owned(),
                ref_key_type_name: ref_key_type_name.to_owned(),
                attributes_type_name: attrs_type_name.to_owned(),
                attributes: attributes,
                service_name: service_name.to_owned(),
                hosts: hosts_names.clone(),
                global_scope: global_scope,
            };

            let header_tpl_bytes = include_bytes!("templates/adjunct_entity__header.gtmpl");
            let header_code = render_template(
                String::from_utf8_lossy(header_tpl_bytes).as_ref(),
                tpl_ctx.to_owned(),
            )?;

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
            render_file_region!(out_file, "ID", "templates/adjunct_entity_id.gtmpl", tpl_ctx);
            render_file_region!(
                out_file,
                "RefKey",
                "templates/adjunct_entity_ref_key.gtmpl",
                tpl_ctx
            );
            render_file_region!(
                out_file,
                "Attributes",
                "templates/adjunct_entity_attributes.gtmpl",
                tpl_ctx
            );
            //render_file_region!(out_file, "Events", "templates/adjunct_entity_event.gtmpl", tpl_ctx);
            render_file_region!(
                out_file,
                "Service",
                "templates/adjunct_entity_service.gtmpl",
                tpl_ctx
            );

            Ok(())
        } else {
            Err(Box::new(azml::azml::Error::Msg(
                "Unsupported ID type".to_owned(),
            )))
        }
    }

    pub fn generate_adjunct_value_object_codes(
        &self,
        module_name: &String,
        _adj_vo: &adjunct_value_object::AdjunctValueObject,
        adj: &adjunct::Adjunct,
        sym: &symbol::Symbol,
    ) -> Result<(), Box<dyn error::Error>> {
        let base_dir = &self.base_dir;
        let type_name = sym.identifier.to_owned();
        let hosts_names = (&adj.hosts)
            .into_iter()
            .map(|x| x.name.to_owned())
            .collect::<Vec<String>>();
        let base_type_name = if adj.name_is_prepared {
            "".to_owned()
        } else {
            hosts_names.join("")
        };

        let type_name = format!("{}{}", base_type_name, type_name);

        let tpl_ctx = AdjunctValueObjectContext {
            base: self.render_base_context(),
            pkg_name: module_name.to_lowercase(),
            type_name: type_name.to_owned(),
            type_doc_lines: sym.documentation.lines().map(|x| x.to_owned()).collect(),
            primitive_type_name: "".to_owned(),
        };

        let out_tpl_bytes: &[u8];
        out_tpl_bytes = include_bytes!("templates/adjunct_value_object.gtmpl");

        let out_code = render_template(
            String::from_utf8_lossy(out_tpl_bytes).as_ref(),
            tpl_ctx.to_owned(),
        )?;

        fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
        let mut service_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(format!("{}/{}/{}.go", base_dir, module_name, type_name,))?;
        service_file.write_all(out_code.as_bytes())?;

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
    attributes: Vec<AttributeContext>,
    service_name: String,
    hosts: Vec<String>,
    global_scope: bool,
}

#[derive(Clone, Gtmpl)]
struct AdjunctValueObjectContext {
    base: BaseContext,
    pkg_name: String,
    type_name: String,
    type_doc_lines: Vec<String>,
    primitive_type_name: String,
}
