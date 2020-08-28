mod module;

fn main() {
    let source_file_result = module::load_source_file();
    if source_file_result.is_ok() {
        println!("Success! {:?}", source_file_result.unwrap());
    } else {
        println!("Error? {:?}", source_file_result.err());
    }
}
