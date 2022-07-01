//

#[macro_use]
extern crate gtmpl_derive;

extern crate convert_case;

use std::{collections::HashMap, env, io, io::Write, process};

use azml::azml::{compiler, generator_go};

mod codegen;
mod codegen_go;
mod dot;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Arguments required!");
        process::exit(-1)
    }

    let c = compiler::Compiler::new();
    let compilation_state = c.compile_file(args[1].to_owned());

    match &compilation_state {
        Ok(compilation_state) => {
            println!("{:?}", compilation_state);

            let module_identifier = "".to_owned();

            let azfl_pkg_identifier = "github.com/alloyzeus/go-azfl".to_owned();
            let azcore_pkg_path = "/azcore";
            let azid_pkg_path = "/azid";
            let azob_pkg_path = "/azob";
            let azerrs_pkg_path = "/errors";

            use codegen::CodeGenerator;
            let mut go_codegen = codegen_go::GoCodeGenerator {
                base_dir: "testdata/output/go".to_owned(),
                base_pkg: "pkg".to_owned(),
                module_identifier: module_identifier,
                generate_servers: true,
                file_per_struct: false,
                package_urls: HashMap::new(),
                azlib_prefix: "AZx".to_owned(),
                azcore_import: azfl_pkg_identifier.to_string() + azcore_pkg_path,
                azcore_pkg: "azcore".to_owned(),
                azid_import: azfl_pkg_identifier.to_string() + azid_pkg_path,
                azid_pkg: "azid".to_owned(),
                azob_import: azfl_pkg_identifier.to_string() + azob_pkg_path,
                azob_pkg: "azob".to_owned(),
                azerrs_import: azfl_pkg_identifier.to_string() + azerrs_pkg_path,
                azerrs_pkg: "errors".to_owned(),
                compilation_state: None,
                package_identifier: "".to_owned(),
                package_dir_base_name: "".to_owned(),
            };

            let entry_module = compilation_state
                .modules
                .get(&compilation_state.entry_module);
            match entry_module {
                Some(entry_module) => {
                    let mut buf = io::BufWriter::new(Vec::new());
                    dot::write_dot(
                        &mut buf,
                        compilation_state.entry_module.to_owned(),
                        entry_module,
                    )
                    .unwrap();
                    io::stdout().write_all(buf.buffer()).unwrap();

                    if let Some(go_pkg) = entry_module.generator_options.get("go") {
                        if let Some(go_opts) =
                            go_pkg.downcast_ref::<generator_go::GeneratorGoOptions>()
                        {
                            go_codegen.module_identifier = go_opts.package_identifier.to_owned();

                            let mut package_uris: HashMap<String, String> = HashMap::new();
                            for o in go_opts.package_opts.clone() {
                                package_uris.insert(o.identifier.to_owned(), o.uri.to_owned());
                            }
                            go_codegen.package_urls = package_uris;

                            if !go_opts.azfl_package_uri.is_empty() {
                                let azfl_uri = go_opts.azfl_package_uri.to_owned();

                                go_codegen.azcore_import = azfl_uri.to_owned() + azcore_pkg_path;
                                go_codegen.azid_import = azfl_uri.to_owned() + azid_pkg_path;
                                go_codegen.azob_import = azfl_uri.to_owned() + azob_pkg_path;
                                go_codegen.azerrs_import = azfl_uri.to_owned() + azerrs_pkg_path;
                            }
                        }
                    }
                }
                _ => panic!("No entry module"),
            }

            go_codegen.generate_codes(&compilation_state).unwrap();
        }
        Err(err) => println!("Error! {:?}", err),
    }
}
