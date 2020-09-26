//

use std::{env, io, io::Write, process};

use azml::azml::{adjunct::adjunct, entity::entity, error, module, source_file};

mod codegen;
mod codegen_go;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Arguments required!");
        process::exit(-1)
    }

    let source_file_result = source_file::load_from_file(args[1].to_owned());
    match &source_file_result {
        Ok(src) => println!("{:?}", src),
        Err(err) => println!("Error! {:?}", err),
    }

    if let Ok(src) = source_file_result {
        let mut buf = io::BufWriter::new(Vec::new());
        write_dot(&mut buf, &src).unwrap();
        io::stdout().write_all(buf.buffer()).unwrap();

        use codegen::CodeGenerator;
        let go_codegen = codegen_go::GoCodeGenerator {
            base_dir: "testdata/output/go".to_owned(),
        };

        go_codegen
            .generate_module_codes(
                &src.module,
                &module::ModuleDefinition {
                    symbols: src.symbols.to_vec(),
                },
            )
            .unwrap();
    }
}

fn write_dot(w: &mut impl io::Write, src: &source_file::SourceFile) -> Result<(), error::Error> {
    w.write(format!("digraph {} {{\n", src.module).as_bytes())?;
    for symbol in &src.symbols {
        let params = &symbol.parameters;
        if let Some(ent) = params.downcast_ref::<entity::Entity>() {
            ent.write_dot_identifier(w, symbol.identifier.clone())?;
        } else if let Some(adj) = params.downcast_ref::<adjunct::Adjunct>() {
            adj.write_dot_identifier(w, symbol.identifier.clone())?;
        }
    }
    w.write_all(b"\n")?;
    for symbol in &src.symbols {
        let params = &symbol.parameters;
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
