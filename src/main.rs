//

#[macro_use]
extern crate gtmpl_derive;

extern crate convert_case;

use std::{collections::HashMap, env, io, io::Write, process};

use azml::azml::compiler;

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
            let mut package_urls = HashMap::new();
            package_urls.insert(
                "telephony".to_owned(),
                "github.com/alloyzeus/go-modules/telephony".to_owned(),
            );

            let module_identifier = "".to_owned();

            use codegen::CodeGenerator;
            let mut go_codegen = codegen_go::GoCodeGenerator {
                base_dir: "testdata/output/go".to_owned(),
                base_pkg: "pkg".to_owned(),
                module_identifier: module_identifier,
                generate_servers: true,
                file_per_struct: false,
                package_urls: package_urls,
                azlib_prefix: "AZx".to_owned(),
                azcore_import: "github.com/alloyzeus/go-azfl/azfl".to_owned(),
                azcore_pkg: "azfl".to_owned(),
                azid_import: "github.com/alloyzeus/go-azfl/azfl/azid".to_owned(),
                azid_pkg: "azid".to_owned(),
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

                    if let Some(go_pkg) = entry_module.options.get("go_package") {
                        go_codegen.module_identifier = go_pkg.to_owned();
                    }
                }
                _ => panic!("No entry module"),
            }

            go_codegen.generate_codes(&compilation_state).unwrap();
        }
        Err(err) => println!("Error! {:?}", err),
    }
}
