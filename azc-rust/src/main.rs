//

mod adjunct;
mod adjunct_serde;
mod entity;
mod entity_serde;
mod result;
mod source_file;
mod source_file_serde;
mod symbol;
mod symbol_serde;

fn main() {
    let source_file_result = source_file::load_from_file("testdata/iam/user.yaml");
    match source_file_result {
        Ok(src) => println!("{:?}", src),
        Err(err) => println!("Error! {:?}", err),
    }
}
