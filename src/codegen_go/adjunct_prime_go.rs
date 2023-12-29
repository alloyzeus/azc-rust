//

use std::{error, fs, io::Write};

use crate::convert_case::{Case, Casing};

use azml::azml::{
    adjunct::{adjunct, adjunct_prime},
    symbol,
};

use super::{
    adjunct_go::AdjunctHostContext,
    entity_go::AbstractContext,
    ref_key_go::{RefKeyAzidTextContext, RefKeyContext},
    template::render_template,
    BaseContext, GoCodeGenerator, ImportContext,
};

impl GoCodeGenerator {
    pub fn generate_adjunct_prime_codes(
        &self,
        module_name: &String,
        adj_prime: &adjunct_prime::AdjunctPrime,
        adj: &adjunct::Adjunct,
        sym: &symbol::Symbol,
    ) -> Result<(), Box<dyn error::Error>> {
        let pkg_name = module_name.to_lowercase();
        let type_name = sym.identifier.to_owned();
        let type_name_snake = type_name.to_case(Case::Snake);
        //TODO: collect the name with the kind as the default
        let hosts_names = (&adj.hosts)
            .iter()
            .map(|x| String::from(&x.kind))
            .collect::<Vec<String>>();
        let hosts_ctx = (&adj.hosts)
            .iter()
            .map(|x| {
                if x.kind.package_identifier.is_empty() {
                    let mut y = x.to_owned();
                    y.kind.package_identifier = pkg_name.to_owned();
                    return AdjunctHostContext::from(&y);
                }
                AdjunctHostContext::from(x)
            })
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
                        ref_key_type_name: format!("{}ID", x.kind.symbol_name),
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
        let service_name = format!("{}Service", type_name);
        let server_name = format!("{}Server", type_name);
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
            pkg_name: pkg_name.to_owned(),
            pkg_path: self.contract_package_identifier.to_owned(),
            imports,
            type_name: type_name.to_owned(),
            type_name_snake: type_name_snake.to_owned(),
            kind: adj_prime.kind.to_owned(),
            ref_key_enabled: adj_prime.identity.enabled,
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
            service_name: service_name.to_owned(),
            server_name: server_name.to_owned(),
            hosts: hosts_ctx,
            one_to_one: adj.cardinality.at_max_one(),
        };

        let header_tpl_bytes =
            include_bytes!("templates/adjunct_value/adjunct_value__header.gtmpl");
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
                self.contract_package_dir_base_name, type_name_snake
            ))?;
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

        if tpl_ctx.ref_key_enabled {
            render_file_region!(
                out_file,
                "ID",
                "templates/adjunct_prime/adjunct_prime_ref_key.gtmpl",
                tpl_ctx
            );
        }

        render_file_region!(
            out_file,
            "Service",
            "templates/adjunct_prime/adjunct_prime_service.gtmpl",
            tpl_ctx
        );

        // ServiceServerBase
        render_file!(
            format!("{}", self.server_package_dir_base_name),
            format!("{}_server__azgen", type_name_snake),
            "templates/adjunct_prime/adjunct_prime_service_server_base.gtmpl",
            tpl_ctx,
            ""
        );

        Ok(())
    }
}

#[derive(Clone, Gtmpl)]
struct AdjunctPrimeContext {
    base: BaseContext,
    pkg_name: String,
    pkg_path: String,
    imports: Vec<ImportContext>,
    type_name: String,
    type_name_snake: String,
    kind: String,
    ref_key_enabled: bool,
    ref_key_type_name: String,
    ref_key_def: RefKeyContext,
    implements: Vec<AbstractContext>,
    service_name: String,
    server_name: String,
    hosts: Vec<AdjunctHostContext>,
    one_to_one: bool,
}
