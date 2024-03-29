//

use std::{error, fs, io::Write};

use crate::convert_case::{Case, Casing};

use azml::azml::{
    entity::{
        id::id_num,
        lifecycle::{creation, deletion, expiration, lifecycle},
        root_entity,
    },
    symbol,
};

use super::{
    attribute_go::AttributeContext,
    id_num_go::IntegerIdNumContext,
    ref_key_go::{RefKeyAzidTextContext, RefKeyContext},
    template::render_template,
    BaseContext, GoCodeGenerator, ImportContext, ServiceContext,
};

impl GoCodeGenerator {
    pub fn root_entity_server_fields(
        &self,
        module_name: &String,
        ent: &root_entity::RootEntity,
        symbol: &symbol::Symbol,
    ) -> Result<Vec<ServiceContext>, Box<dyn error::Error>> {
        let ctx = self.render_context(module_name, ent, symbol)?;
        Ok(vec![ServiceContext {
            field_name: ctx.service_name.to_owned(),
            type_name: ctx.service_name.to_owned(),
            server_name: ctx.server_name.to_owned(),
        }])
    }

    fn render_context(
        &self,
        module_name: &String,
        ent: &root_entity::RootEntity,
        symbol: &symbol::Symbol,
    ) -> Result<EntityContext, Box<dyn error::Error>> {
        let id_num_def = &ent.id.num.definition;

        if let Some(id_int) = id_num_def.downcast_ref::<id_num::IntegerIdNum>() {
            let type_name = symbol.identifier.to_owned();
            let type_name_snake = type_name.to_case(Case::Snake);
            let id_num_type_name = format!("{}IDNum", type_name);
            let ref_key_type_name = format!("{}ID", type_name);
            let attrs_type_name = format!("{}AttrSet", type_name);
            let event_interface_name = format!("{}Event", type_name);
            let service_name = format!("{}Service", type_name);
            let server_name = format!("{}Server", type_name);
            //let service_name_snake = service_name.to_case(Case::Snake);
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
                            ref_key_type_name: format!("{}ID", x.kind.symbol_name),
                            singular: a.singular,
                            is_system: x.kind.package_identifier == "_azsys",
                        }),
                    }
                })
                .filter(|x| !x.is_none())
                .map(|x| x.unwrap())
                .collect::<Vec<AbstractContext>>();

            Ok(EntityContext {
                base: self.render_base_context(),
                pkg_name: module_name.to_lowercase(),
                pkg_path: self.contract_package_identifier.to_owned(),
                imports,
                type_name: type_name.to_owned(),
                type_name_snake: type_name_snake.to_owned(),
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
                attributes,
                event_interface_name: event_interface_name.to_owned(),
                service_name: service_name.to_owned(),
                server_name: server_name.to_owned(),
                lifecycle: (&ent.lifecycle).into(),
            })
        } else {
            Err(Box::new(azml::azml::Error::Msg(
                "Unsupported id_num type".to_owned(),
            )))
        }
    }

    pub fn generate_root_entity_codes(
        &self,
        module_name: &String,
        ent: &root_entity::RootEntity,
        symbol: &symbol::Symbol,
    ) -> Result<(), Box<dyn error::Error>> {
        let tpl_ctx = self.render_context(module_name, ent, symbol)?;

        let header_tpl_bytes = include_bytes!("templates/entity/entity__header.gtmpl");
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
                self.contract_package_dir_base_name, tpl_ctx.type_name_snake
            ))?;
        out_file.write_all(header_code.as_bytes())?;
        out_file.write_all(format!("\n// Entity {}.\n", tpl_ctx.type_name).as_bytes())?;
        if !tpl_ctx.type_doc_lines.is_empty() {
            out_file.write_all("//\n".as_bytes())?;
            for x in &tpl_ctx.type_doc_lines {
                if x.is_empty() {
                    out_file.write_all("//\n".as_bytes())?;
                } else {
                    out_file.write_all("// ".as_bytes())?;
                    out_file.write_all(x.as_bytes())?;
                    out_file.write_all("\n".as_bytes())?;
                }
            }
        }
        render_file_region!(
            out_file,
            "ID",
            "templates/entity/entity_ref_key.gtmpl",
            tpl_ctx
        );
        render_file_region!(
            out_file,
            "IDNum",
            "templates/entity/entity_id_num.gtmpl",
            tpl_ctx
        );
        render_file_region!(
            out_file,
            "AttrSet",
            "templates/entity/entity_attributes.gtmpl",
            tpl_ctx
        );
        // render_file_region!(
        //     out_file,
        //     "Events",
        //     "templates/entity/entity_event.gtmpl",
        //     tpl_ctx
        // );
        render_file_region!(
            out_file,
            "Instance",
            "templates/entity/entity_instance.gtmpl",
            tpl_ctx
        );
        render_file_region!(
            out_file,
            "Service",
            "templates/entity/entity_service.gtmpl",
            tpl_ctx
        );

        // // ServiceClient
        // render_file!(
        //     format!("{}/client", self.package_dir_base_name),
        //     format!("{}ClientBase", service_name),
        //     "templates/entity/entity_service_client_base.gtmpl",
        //     tpl_ctx,
        //     ""
        // );

        // ServiceServerBase
        render_file!(
            format!("{}", self.server_package_dir_base_name),
            format!("{}_server__azgen", tpl_ctx.type_name_snake),
            "templates/entity/entity_service_server_base.gtmpl",
            tpl_ctx,
            ""
        );

        // // ServiceServer
        // render_file!(
        //     format!("{}", self.server_package_dir_base_name),
        //     format!("{}Server", service_name),
        //     "templates/entity/entity_service_server.gtmpl",
        //     tpl_ctx,
        //     ""
        // );

        Ok(())
    }
}

#[derive(Clone, Gtmpl)]
pub struct AbstractContext {
    pub type_name: String,
    pub ref_key_type_name: String,
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
    server_name: String,
    lifecycle: EntityLifecycleContext,
}

#[derive(Clone, Gtmpl)]
pub struct EntityLifecycleContext {
    creation: EntityCreationContext,
    deletion: EntityDeletionContext,
    expiration: EntityExpirationContext,
}

impl From<&lifecycle::Lifecycle> for EntityLifecycleContext {
    fn from(s: &lifecycle::Lifecycle) -> Self {
        Self {
            creation: (&s.creation).into(),
            deletion: (&s.deletion).into(),
            expiration: (&s.expiration).into(),
        }
    }
}

#[derive(Clone, Gtmpl)]
struct EntityCreationContext {
    allow_cross_process_callers: bool,
}

impl From<&creation::Creation> for EntityCreationContext {
    fn from(s: &creation::Creation) -> Self {
        Self {
            allow_cross_process_callers: s.authorization.same_realm.allow.is_not_disallow(),
        }
    }
}

#[derive(Clone, Gtmpl)]
struct EntityDeletionContext {
    enabled: bool,
    notes: EntityDeletionNotesContext,
}

impl From<&deletion::Deletion> for EntityDeletionContext {
    fn from(s: &deletion::Deletion) -> Self {
        Self {
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
    fn from(s: &deletion::DeletionNotes) -> Self {
        Self {
            enabled: s.enabled,
            required: s.required,
        }
    }
}

#[derive(Clone, Gtmpl)]
struct EntityExpirationContext {
    enabled: bool,
}

impl From<&expiration::Expiration> for EntityExpirationContext {
    fn from(s: &expiration::Expiration) -> Self {
        Self { enabled: s.enabled }
    }
}
