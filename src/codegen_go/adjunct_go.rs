//

use std::{error, fs, io::Write};

use crate::{
    codegen_go::{
        attribute_go::AttributeContext,
        entity_go::AbstractContext,
        id_num_go::IntegerIdNumContext,
        ref_key_go::{RefKeyAzidTextContext, RefKeyContext},
        BaseContext, GoCodeGenerator, ImportContext,
    },
    convert_case::{Case, Casing},
};

use azml::azml::{
    adjunct::{adjunct, adjunct_entity, adjunct_value},
    entity::entity,
    symbol,
};

use crate::codegen_go::template::render_template;

//TODO:
// - if there are more than one hosts with the same type ensure that
//   each has an assigned name and ensure to generate code to ensure that
//   they are ordered

impl GoCodeGenerator {
    pub fn generate_adjunct_codes(
        &self,
        module_name: &String,
        adj: &adjunct::Adjunct,
        sym: &symbol::Symbol,
    ) -> Result<(), Box<dyn error::Error>> {
        if let Some(adj_def) = adj
            .definition
            .downcast_ref::<adjunct_entity::AdjunctEntity>()
        {
            self.generate_adjunct_entity_codes(module_name, adj_def, &adj, &sym)?;
            Ok(())
        } else if let Some(adj_def) = adj.definition.downcast_ref::<adjunct_value::AdjunctPrime>() {
            self.generate_adjunct_value_codes(module_name, adj_def, &adj, &sym)?;
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
        let type_name = sym.identifier.to_owned();
        //TODO: collect the name with the kind as the default
        let hosts_names = (&adj.hosts)
            .iter()
            .map(|x| String::from(&x.kind))
            .collect::<Vec<String>>();
        //TODO: generate code for each abstract host implementation
        let hosts = (&adj.hosts)
            .iter()
            .map(|x| self.lookup_entity(x.kind.clone()))
            .collect::<Vec<Option<Box<&dyn entity::Entity>>>>();
        //TODO: move this to the compiler
        let non_resolved = hosts.iter().any(|x| x.is_none());
        if !non_resolved {
            return Err(Box::new(azml::azml::Error::Msg(
                //TODO: which host?
                "One of the hosts is not resolvable or not an entity".to_owned(),
            )));
        }
        //TODO: error when any abstract is unresolvable
        let abstracts = adj_ent
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

        // If the adjunct is globally addressable, i.e., an instance's
        // id-num is unique system-wide, it must not derive its hosts' name
        // by default.
        // And also, the RefKey is just a typedef of id-num.
        let global_scope = adjunct_entity::AdjunctEntityScope::Global == adj_ent.scope;
        let base_type_name = if adj.name_is_prepared || global_scope {
            "".to_owned()
        } else {
            hosts_names.join("")
        };

        let type_name = format!("{}{}", base_type_name, type_name);
        let type_doc_lines: Vec<String> = sym.documentation.lines().map(|x| x.to_owned()).collect();
        let imports = sym
            .definition
            .collect_symbol_refs()
            .iter()
            .filter(|x| !x.package_identifier.is_empty())
            .map(|x| ImportContext {
                alias: x.package_identifier.to_owned(),
                url: self.resolve_import(&x.package_identifier),
            })
            .collect();
        let id_num_def = &adj_ent.id.num.definition;

        if let Some(id_int) = id_num_def.downcast_ref::<adjunct_entity::AdjunctEntityIdNumInteger>()
        {
            let id_num_type_name = format!("{}IDNum", type_name);
            let ref_key_type_name = format!("{}ID", type_name);
            let attrs_type_name = format!("{}Attributes", type_name);
            let service_name = format!("{}Service", type_name);
            let attributes: Vec<AttributeContext> = (&adj_ent.attributes)
                .iter()
                .map(|attr| attr.into())
                .collect();

            if !id_int.bitfield.inherits.is_empty() {
                // let host = self.get_entity(module_name.to_owned(), adj.hosts[0].name.to_owned());
                // println!("Adjunct {} host {:?}", type_name, host);
            }

            let tpl_ctx = AdjunctEntityContext {
                base: self.render_base_context(),
                pkg_name: module_name.to_lowercase(),
                pkg_path: self.package_identifier.to_owned(),
                type_name: type_name.to_owned(),
                type_name_snake: type_name.to_case(Case::Snake),
                imports: imports,
                id_num_type_name: id_num_type_name.to_owned(),
                id_num_def: id_int.into(),
                ref_key_type_name: ref_key_type_name.to_owned(),
                ref_key_def: RefKeyContext {
                    azid_text: RefKeyAzidTextContext {
                        prefix: if adj_ent.id.ref_key.azid_text.prefix.is_empty() {
                            type_name.to_owned()
                        } else {
                            adj_ent.id.ref_key.azid_text.prefix.to_owned()
                        },
                    },
                },
                implements: abstracts,
                attributes_type_name: attrs_type_name.to_owned(),
                attributes: attributes,
                service_name: service_name.to_owned(),
                hosts: hosts_names.clone(),
                global_scope: global_scope,
            };

            let header_tpl_bytes =
                include_bytes!("templates/adjunct_entity/adjunct_entity__header.gtmpl");
            let header_code = render_template(
                String::from_utf8_lossy(header_tpl_bytes).as_ref(),
                tpl_ctx.to_owned(),
            )?;

            let mut out_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(format!("{}/{}.go", self.package_dir_base_name, type_name))?;
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
                "IDNum",
                "templates/adjunct_entity/adjunct_entity_id_num.gtmpl",
                tpl_ctx
            );
            render_file_region!(
                out_file,
                "ID",
                "templates/adjunct_entity/adjunct_entity_ref_key.gtmpl",
                tpl_ctx
            );
            // render_file_region!(
            //     out_file,
            //     "Attributes",
            //     "templates/adjunct_entity/adjunct_entity_attributes.gtmpl",
            //     tpl_ctx
            // );
            // render_file_region!(out_file, "Events", "templates/adjunct_entity/adjunct_entity_event.gtmpl", tpl_ctx);
            // render_file_region!(
            //     out_file,
            //     "Service",
            //     "templates/adjunct_entity/adjunct_entity_service.gtmpl",
            //     tpl_ctx
            // );

            // ServiceServerBase
            render_file!(
                format!("{}server", self.package_dir_base_name),
                format!("{}ServerBase", service_name),
                "templates/adjunct_entity/adjunct_entity_service_server_base.gtmpl",
                tpl_ctx,
                ""
            );

            Ok(())
        } else {
            Err(Box::new(azml::azml::Error::Msg(
                "Unsupported id_num type".to_owned(),
            )))
        }
    }

    pub fn generate_adjunct_value_codes(
        &self,
        module_name: &String,
        adj_prime: &adjunct_value::AdjunctPrime,
        adj: &adjunct::Adjunct,
        sym: &symbol::Symbol,
    ) -> Result<(), Box<dyn error::Error>> {
        let type_name = sym.identifier.to_owned();
        //TODO: collect the name with the kind as the default
        let hosts_names = (&adj.hosts)
            .iter()
            .map(|x| String::from(&x.kind))
            .collect::<Vec<String>>();
        let hosts_ctx = (&adj.hosts)
            .iter()
            .map(|x| AdjunctHostContext::from(x))
            .collect::<Vec<AdjunctHostContext>>();
        let base_type_name = if adj.name_is_prepared {
            "".to_owned()
        } else {
            (&hosts_names)
                .iter()
                .map(|x| {
                    let v = x.split(".").last();
                    if let Some(i) = v {
                        i.to_owned()
                    } else {
                        x.to_owned()
                    }
                })
                .collect::<Vec<String>>()
                .join("")
        };
        //TODO: error when any abstract is unresolvable
        let abstracts = adj_prime
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

        let type_name = format!("{}{}", base_type_name, type_name);
        let type_doc_lines: Vec<String> = sym.documentation.lines().map(|x| x.to_owned()).collect();
        let imports = sym
            .definition
            .collect_symbol_refs()
            .iter()
            .filter(|x| !x.package_identifier.is_empty())
            .map(|x| ImportContext {
                alias: x.package_identifier.to_owned(),
                url: self.resolve_import(&x.package_identifier),
            })
            .collect();

        let ref_key_type_name = format!("{}ID", type_name);

        let tpl_ctx = AdjunctPrimeContext {
            base: self.render_base_context(),
            pkg_name: module_name.to_lowercase(),
            imports: imports,
            type_name: type_name.to_owned(),
            ref_key_type_name: ref_key_type_name.to_owned(),
            ref_key_def: RefKeyContext {
                azid_text: RefKeyAzidTextContext {
                    prefix: if adj_prime.identity.prefix.is_empty() {
                        type_name.to_owned()
                    } else {
                        adj_prime.identity.prefix.to_owned()
                    },
                },
            },
            implements: abstracts,
            hosts: hosts_ctx,
        };

        let header_tpl_bytes =
            include_bytes!("templates/adjunct_value/adjunct_value__header.gtmpl");
        let header_code = render_template(
            String::from_utf8_lossy(header_tpl_bytes).as_ref(),
            tpl_ctx.to_owned(),
        )?;

        let mut out_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(format!("{}/{}.go", self.package_dir_base_name, type_name))?;
        out_file.write_all(header_code.as_bytes())?;
        out_file.write_all(
            format!(
                "\n// Adjunct-prime {} of {}.\n",
                type_name,
                hosts_names.join(", ")
            )
            .as_bytes(),
        )?;
        if !type_doc_lines.is_empty() {
            out_file.write_all("//\n".as_bytes())?;
            for x in type_doc_lines {
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
            "templates/adjunct_value/adjunct_value_ref_key.gtmpl",
            tpl_ctx
        );

        Ok(())
    }
}

#[derive(Clone, Gtmpl)]
struct AdjunctEntityContext {
    base: BaseContext,
    pkg_name: String,
    pkg_path: String,
    imports: Vec<ImportContext>,
    type_name: String,
    type_name_snake: String,
    id_num_type_name: String,
    id_num_def: IntegerIdNumContext,
    ref_key_type_name: String,
    ref_key_def: RefKeyContext,
    implements: Vec<AbstractContext>,
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
    hosts: Vec<String>,
}

#[derive(Clone, Gtmpl)]
struct AdjunctPrimeContext {
    base: BaseContext,
    pkg_name: String,
    imports: Vec<ImportContext>,
    type_name: String,
    ref_key_type_name: String,
    ref_key_def: RefKeyContext,
    implements: Vec<AbstractContext>,
    hosts: Vec<AdjunctHostContext>,
}

#[derive(Clone, Gtmpl)]
struct AdjunctHostContext {
    full_type_name: String,
    bare_type_name: String,
    identifier_name: String,
}

impl From<&adjunct::AdjunctHost> for AdjunctHostContext {
    fn from(x: &adjunct::AdjunctHost) -> Self {
        Self {
            full_type_name: if x.kind.package_identifier.is_empty() {
                x.kind.symbol_name.to_owned()
            } else {
                format!("{}.{}", x.kind.package_identifier, x.kind.symbol_name)
            },
            bare_type_name: x.kind.symbol_name.to_owned(),
            identifier_name: if x.name.is_empty() {
                x.kind.symbol_name.to_owned()
            } else {
                x.name.to_owned()
            },
        }
    }
}
