//

use std::{error, fs, io::Write};

use crate::{
    codegen_go::{
        attribute_go::AttributeContext,
        id_num_go::IntegerIdNumContext,
        ref_key_go::{RefKeyAzidTextContext, RefKeyContext},
        BaseContext, GoCodeGenerator, ImportContext,
    },
    convert_case::{Case, Casing},
};

use azml::azml::{
    entity::{
        id::id_num,
        lifecycle::{creation::creation, deletion::deletion, lifecycle},
        root_entity,
    },
    symbol,
};

use crate::codegen_go::template::render_template;

impl GoCodeGenerator {
    pub fn generate_root_entity_codes(
        &self,
        module_name: &String,
        ent: &root_entity::RootEntity,
        symbol: &symbol::Symbol,
    ) -> Result<(), Box<dyn error::Error>> {
        let type_name = symbol.identifier.to_owned();
        let id_num_def = &ent.id.num.definition;

        if let Some(id_int) = id_num_def.downcast_ref::<id_num::IntegerIdNum>() {
            let id_num_type_name = format!("{}IDNum", type_name);
            let ref_key_type_name = format!("{}RefKey", type_name);
            let attrs_type_name = format!("{}Attributes", type_name);
            let event_interface_name = format!("{}Event", type_name);
            let service_name = format!("{}Service", type_name);
            let type_doc_lines: Vec<String> =
                symbol.documentation.lines().map(|x| x.to_owned()).collect();
            let attributes: Vec<AttributeContext> =
                (&ent.attributes).iter().map(|attr| attr.into()).collect();
            let imports = symbol
                .definition
                .collect_symbol_refs()
                .iter()
                .filter(|x| !x.package_identifier.is_empty())
                .map(|x| ImportContext {
                    alias: x.package_identifier.to_owned(),
                    url: self.resolve_import(&x.package_identifier),
                })
                .collect();
            //TODO: error when any abstract is unresolvable
            let abstracts = ent
                .implements
                .iter()
                .map(|x| {
                    let y = self.lookup_abstract(x.kind.clone());
                    match y {
                        None => None,
                        Some(a) => Some(AbstractContext {
                            type_name: x.kind.symbol_name.to_owned(),
                            singular: a.singular,
                            is_system: x.kind.package_identifier == "_azsys",
                        }),
                    }
                })
                .filter(|x| !x.is_none())
                .map(|x| x.unwrap())
                .collect::<Vec<AbstractContext>>();

            let tpl_ctx = EntityContext {
                base: self.render_base_context(),
                pkg_name: module_name.to_lowercase(),
                pkg_path: self.package_identifier.to_owned(),
                imports: imports,
                type_name: type_name.to_owned(),
                type_name_snake: type_name.to_case(Case::Snake),
                type_doc_lines: type_doc_lines.clone(),
                id_num_type_name: id_num_type_name.to_owned(),
                id_num_def: id_int.into(),
                ref_key_type_name: ref_key_type_name.to_owned(),
                ref_key_def: RefKeyContext {
                    azid_text: RefKeyAzidTextContext {
                        prefix: if ent.id.ref_key.azid_text.prefix.is_empty() {
                            type_name.to_owned()
                        } else {
                            ent.id.ref_key.azid_text.prefix.to_owned()
                        },
                    },
                },
                implements: abstracts,
                attributes_type_name: attrs_type_name.to_owned(),
                attributes: attributes,
                event_interface_name: event_interface_name.to_owned(),
                service_name: service_name.to_owned(),
                lifecycle: (&ent.lifecycle).into(),
            };

            let header_tpl_bytes = include_bytes!("templates/entity__header.gtmpl");
            let header_code = render_template(
                String::from_utf8_lossy(header_tpl_bytes).as_ref(),
                tpl_ctx.to_owned(),
            )?;

            let mut out_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(format!("{}/{}.go", self.package_dir_base_name, type_name))?;
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
            render_file_region!(out_file, "IDNum", "templates/entity_id_num.gtmpl", tpl_ctx);
            render_file_region!(
                out_file,
                "RefKey",
                "templates/entity_ref_key.gtmpl",
                tpl_ctx
            );
            // render_file_region!(
            //     out_file,
            //     "Attributes",
            //     "templates/entity_attributes.gtmpl",
            //     tpl_ctx
            // );
            // render_file_region!(out_file, "Events", "templates/entity_event.gtmpl", tpl_ctx);
            render_file_region!(
                out_file,
                "Instance",
                "templates/entity_instance.gtmpl",
                tpl_ctx
            );
            render_file_region!(
                out_file,
                "Service",
                "templates/entity_service.gtmpl",
                tpl_ctx
            );
            // render_file_region!(
            //     out_file,
            //     "ServiceBase",
            //     "templates/entity_service_base.gtmpl",
            //     tpl_ctx
            // );

            // // ServiceClient
            // render_file!(
            //     format!("{}/client", self.package_dir_base_name),
            //     format!("{}ClientBase", service_name),
            //     "templates/entity_service_client_base.gtmpl",
            //     tpl_ctx,
            //     ""
            // );

            // ServiceServer
            render_file!(
                format!("{}server", self.package_dir_base_name),
                format!("{}ServerBase", service_name),
                "templates/entity_service_server_base.gtmpl",
                tpl_ctx,
                ""
            );

            // // ServiceServer
            // render_file!(
            //     format!("{}server", self.package_dir_base_name),
            //     format!("{}Server", service_name),
            //     "templates/entity_service_server.gtmpl",
            //     tpl_ctx,
            //     ""
            // );

            Ok(())
        } else {
            Err(Box::new(azml::azml::Error::Msg(
                "Unsupported id_num type".to_owned(),
            )))
        }
    }
}

#[derive(Clone, Gtmpl)]
pub struct AbstractContext {
    pub type_name: String,
    pub singular: bool,
    pub is_system: bool,
    //TODO: attributes
}

#[derive(Clone, Gtmpl)]
struct EntityContext {
    base: BaseContext,
    pkg_name: String,
    pkg_path: String,
    imports: Vec<ImportContext>,
    type_name: String,
    type_name_snake: String,
    type_doc_lines: Vec<String>,
    id_num_type_name: String,
    id_num_def: IntegerIdNumContext,
    ref_key_type_name: String,
    ref_key_def: RefKeyContext,
    implements: Vec<AbstractContext>,
    attributes_type_name: String,
    attributes: Vec<AttributeContext>,
    event_interface_name: String,
    service_name: String,
    lifecycle: EntityLifecycleContext,
}

#[derive(Clone, Gtmpl)]
struct EntityLifecycleContext {
    creation: EntityCreationContext,
    deletion: EntityDeletionContext,
}

impl From<&lifecycle::Lifecycle> for EntityLifecycleContext {
    fn from(s: &lifecycle::Lifecycle) -> EntityLifecycleContext {
        EntityLifecycleContext {
            creation: (&s.creation).into(),
            deletion: (&s.deletion).into(),
        }
    }
}

#[derive(Clone, Gtmpl)]
struct EntityCreationContext {
    allow_cross_process_callers: bool,
}

impl From<&creation::Creation> for EntityCreationContext {
    fn from(s: &creation::Creation) -> EntityCreationContext {
        EntityCreationContext {
            allow_cross_process_callers: s.allow_cross_process_callers,
        }
    }
}

#[derive(Clone, Gtmpl)]
struct EntityDeletionContext {
    enabled: bool,
    notes: EntityDeletionNotesContext,
}

impl From<&deletion::Deletion> for EntityDeletionContext {
    fn from(s: &deletion::Deletion) -> EntityDeletionContext {
        EntityDeletionContext {
            enabled: s.enabled,
            notes: (&s.notes).into(),
        }
    }
}

#[derive(Clone, Gtmpl)]
struct EntityDeletionNotesContext {
    enabled: bool,
    required: bool,
}

impl From<&deletion::DeletionNotes> for EntityDeletionNotesContext {
    fn from(s: &deletion::DeletionNotes) -> EntityDeletionNotesContext {
        EntityDeletionNotesContext {
            enabled: s.enabled,
            required: s.required,
        }
    }
}
