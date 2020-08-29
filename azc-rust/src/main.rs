//

mod adjunct;
mod adjunct_serde;
mod entity;
mod entity_serde;
mod result;
mod source_file;
mod source_file_serde;
mod symbol;
mod symbol_kind;
mod symbol_serde;

fn main() {
    let source_file_result = source_file::load_from_file("testdata/iam/user.yaml");
    match &source_file_result {
        Ok(src) => println!("{:?}", src),
        Err(err) => println!("Error! {:?}", err),
    }

    if let Ok(src) = source_file_result {
        for symbol in src.symbols {
            if let Some(params) = symbol.parameters {
                if let Some(ent) = params.downcast_ref::<entity::Entity>() {
                    println!("Ent {} {:?}", symbol.identifier, ent);
                } else if let Some(adj) = params.downcast_ref::<adjunct::Adjunct>() {
                    println!("Adj {} {:?}", symbol.identifier, adj);
                }
            }
        }
    }
}
