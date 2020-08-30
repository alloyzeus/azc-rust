//

mod adjunct;
mod adjunct_serde;
mod arity;
mod arity_serde;
mod entity;
mod entity_serde;
mod mixin;
mod mixin_serde;
mod mixins;
mod result;
mod source_file;
mod source_file_serde;
mod symbol;
mod symbol_kind;
mod symbol_serde;

#[macro_use]
extern crate mopa;

fn main() {
    let source_file_result = source_file::load_from_file("testdata/iam/user.yaml");
    match &source_file_result {
        Ok(src) => println!("{:?}", src),
        Err(err) => println!("Error! {:?}", err),
    }

    if let Ok(src) = source_file_result {
        println!("digraph iam {{");
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
        for ent in &self.entities {
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
