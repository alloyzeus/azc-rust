//

mod adjunct;
mod adjunct_serde;
mod base;
mod entity;
mod entity_serde;
mod mixin;
mod mixin_serde;
mod mixins;
mod source_file;
mod source_file_serde;
mod symbol;
mod symbol_kind;
mod symbol_serde;

#[macro_use]
extern crate mopa;
use std::{env, process};

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
        println!("digraph {} {{", src.module);
        for symbol in &src.symbols {
            if let Some(params) = &symbol.parameters {
                if let Some(ent) = params.downcast_ref::<entity::Entity>() {
                    ent.write_dot_identifier(symbol.identifier.clone());
                } else if let Some(adj) = params.downcast_ref::<adjunct::Adjunct>() {
                    adj.write_dot_identifier(symbol.identifier.clone());
                }
            }
        }
        println!();
        for symbol in src.symbols {
            if let Some(params) = symbol.parameters {
                if let Some(ent) = params.downcast_ref::<entity::Entity>() {
                    ent.write_dot_relationships(symbol.identifier);
                } else if let Some(adj) = params.downcast_ref::<adjunct::Adjunct>() {
                    adj.write_dot_relationships(symbol.identifier);
                }
            }
        }
        println!("}}");
    }
}

trait DotObject {
    fn write_dot_identifier(&self, identifier: String);
    fn write_dot_relationships(&self, identifier: String);
}

impl DotObject for adjunct::Adjunct {
    fn write_dot_identifier(&self, identifier: String) {
        println!("  {} [shape=ellipse]", identifier);
    }
    fn write_dot_relationships(&self, identifier: String) {
        for ent in &self.hosts {
            println!("  {} -> {}", identifier, ent.name);
        }
    }
}

impl DotObject for entity::Entity {
    fn write_dot_identifier(&self, identifier: String) {
        println!("  {} [shape=rect]", identifier);
    }
    fn write_dot_relationships(&self, _identifier: String) {
        // Do nothing
    }
}
