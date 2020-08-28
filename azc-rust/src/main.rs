//

mod entity;
mod entity_serde;
mod module;

fn main() {
    let source_file_result = module::load_source_file_from_string(
        r#"
            module: iam
            symbols:
              - name: User
                kind: entity
                parameters:
                  description: "Test yo!"
              - name: Application
                kind: entity
            "#,
    );
    if source_file_result.is_ok() {
        println!("Success! {:?}", source_file_result.unwrap());
    } else {
        println!("Error? {:?}", source_file_result.err());
    }
}
