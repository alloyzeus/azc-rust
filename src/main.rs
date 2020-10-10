//

#[macro_use]
extern crate gtmpl_derive;

use std::{collections::HashMap, env, io, io::Write, process};

use azml::azml::{adjunct::adjunct, compiler, entity::entity, error, module};

mod codegen;
mod codegen_go;

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

            use codegen::CodeGenerator;
            let go_codegen = codegen_go::GoCodeGenerator {
                base_dir: "testdata/output/go".to_owned(),
                module_identifier: "github.com/alloyzeus/go-examples".to_owned(),
                file_per_struct: false,
                package_urls: package_urls,
                azlib_prefix: "AZx".to_owned(),
                azcore_import: "github.com/alloyzeus/go-azcore/azcore".to_owned(),
                azcore_pkg: "azcore".to_owned(),
            };

            let entry_module = compilation_state
                .modules
                .get(&compilation_state.entry_module);
            match entry_module {
                Some(entry_module) => {
                    let mut buf = io::BufWriter::new(Vec::new());
                    write_dot(
                        &mut buf,
                        compilation_state.entry_module.to_owned(),
                        entry_module,
                    )
                    .unwrap();
                    io::stdout().write_all(buf.buffer()).unwrap();
                }
                _ => panic!("No entry module"),
            }

            go_codegen.generate_codes(&compilation_state).unwrap();
        }
        Err(err) => println!("Error! {:?}", err),
    }
}

fn write_dot(
    w: &mut impl io::Write,
    module_name: String,
    module_def: &module::ModuleDefinition,
) -> Result<(), error::Error> {
    w.write(format!("digraph {} {{\n", module_name).as_bytes())?;
    for symbol in &module_def.symbols {
        let params = &symbol.definition;
        if let Some(ent) = params.downcast_ref::<entity::Entity>() {
            ent.write_dot_identifier(w, symbol.identifier.clone())?;
        } else if let Some(adj) = params.downcast_ref::<adjunct::Adjunct>() {
            adj.write_dot_identifier(w, symbol.identifier.clone())?;
        }
    }
    w.write_all(b"\n")?;
    for symbol in &module_def.symbols {
        let params = &symbol.definition;
        if let Some(ent) = params.downcast_ref::<entity::Entity>() {
            ent.write_dot_relationships(w, symbol.identifier.clone())?;
        } else if let Some(adj) = params.downcast_ref::<adjunct::Adjunct>() {
            adj.write_dot_relationships(w, symbol.identifier.clone())?;
        }
    }
    w.write_all(b"}\n")?;

    Ok(())
}

trait DotNode {
    fn write_dot_identifier(
        &self,
        w: &mut impl io::Write,
        identifier: String,
    ) -> Result<(), io::Error>;
    fn write_dot_relationships(
        &self,
        w: &mut impl io::Write,
        identifier: String,
    ) -> Result<(), io::Error>;
}

impl DotNode for adjunct::Adjunct {
    fn write_dot_identifier(
        &self,
        w: &mut impl io::Write,
        identifier: String,
    ) -> Result<(), io::Error> {
        w.write(format!("  {} [shape=ellipse]\n", identifier).as_bytes())?;
        Ok(())
    }
    fn write_dot_relationships(
        &self,
        w: &mut impl io::Write,
        identifier: String,
    ) -> Result<(), io::Error> {
        for ent in &self.hosts {
            w.write(format!("  {} -> {}\n", identifier, ent.name).as_bytes())?;
        }
        Ok(())
    }
}

impl DotNode for entity::Entity {
    fn write_dot_identifier(
        &self,
        w: &mut impl io::Write,
        identifier: String,
    ) -> Result<(), io::Error> {
        w.write(format!("  {} [shape=rect]\n", identifier).as_bytes())?;
        Ok(())
    }
    fn write_dot_relationships(
        &self,
        _w: &mut impl io::Write,
        _identifier: String,
    ) -> Result<(), io::Error> {
        // Do nothing
        Ok(())
    }
}
