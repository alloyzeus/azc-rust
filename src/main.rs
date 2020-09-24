//

use mhtemplate;
use std::{env, io, io::Write, process};

use azml::azml::{
    adjunct,
    entity::{entity, entity_id_integer},
    error, source_file,
};

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

        for symbol in &src.symbols {
            if let Some(params) = &symbol.parameters {
                if let Some(ent) = params.downcast_ref::<entity::Entity>() {
                    generate_entity_codes(ent, symbol.identifier.clone());
                }
            }
        }
    }
}

fn write_dot(w: &mut impl io::Write, src: &source_file::SourceFile) -> Result<(), error::Error> {
    w.write(format!("digraph {} {{\n", src.module).as_bytes())?;
    for symbol in &src.symbols {
        if let Some(params) = &symbol.parameters {
            if let Some(ent) = params.downcast_ref::<entity::Entity>() {
                ent.write_dot_identifier(w, symbol.identifier.clone())?;
            } else if let Some(adj) = params.downcast_ref::<adjunct::Adjunct>() {
                adj.write_dot_identifier(w, symbol.identifier.clone())?;
            }
        }
    }
    w.write_all(b"\n")?;
    for symbol in &src.symbols {
        if let Some(params) = &symbol.parameters {
            if let Some(ent) = params.downcast_ref::<entity::Entity>() {
                ent.write_dot_relationships(w, symbol.identifier.clone())?;
            } else if let Some(adj) = params.downcast_ref::<adjunct::Adjunct>() {
                adj.write_dot_relationships(w, symbol.identifier.clone())?;
            }
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

fn generate_entity_codes(ent: &entity::Entity, identifier: String) {
    if let Some(id_def) = &ent.id.parameters {
        if let Some(id_int) = id_def.downcast_ref::<entity_id_integer::EntityIdInteger>() {
            let id_size = if id_int.space < 16 {
                16
            } else if id_int.space < 32 {
                32
            } else if id_int.space < 64 {
                64
            } else {
                -1 //TODO: error. we won't need this here. generators receive clean data.
            };

            let id_type_name = format!("{}ID", identifier);
            let id_type_primitive = format!("int{}", id_size);
            let service_name = format!("{}Service", identifier);

            let mut mht_ctx = mhtemplate::Context::new();
            mht_ctx["IDENTIFIER"] = identifier;
            mht_ctx["ID_TYPE_NAME"] = id_type_name;
            mht_ctx["ID_TYPE_PRIMITIVE"] = id_type_primitive;
            mht_ctx["SERVICE_NAME"] = service_name;

            let mh_tpl = mhtemplate::TemplateFactory::new(
                "// {{$ID_TYPE_NAME}} is used to identify an instance of {{$IDENTIFIER}}.\n\
                type {{$ID_TYPE_NAME}} {{$ID_TYPE_PRIMITIVE}}\n\
                \n\
                // {{$ID_TYPE_NAME}}Zero is the zero value for entity {{$IDENTIFIER}}.\n\
                const {{$ID_TYPE_NAME}}Zero = {{$ID_TYPE_NAME}}(0)\n\
                \n\
                func {{$ID_TYPE_NAME}}FromPrimitiveValue(v {{$ID_TYPE_PRIMITIVE}}) {{$ID_TYPE_NAME}} { return {{$ID_TYPE_NAME}}(v) }\n\
                func (id {{$ID_TYPE_NAME}}) PrimitiveValue() {{$ID_TYPE_PRIMITIVE}} { return {{$ID_TYPE_PRIMITIVE}}(id) }\n\
                \n\
                type {{$SERVICE_NAME}} interface {\n\
                \t// TODO\n\
                }\n\
                \n\
                type {{$SERVICE_NAME}}Server struct {\n\
                \t// TODO\n\
                }\n\
                \n").parse().unwrap();

            let rx = mh_tpl.evaluate(&mut mht_ctx).unwrap();
            print!("{}", rx);
        }
    }
}
