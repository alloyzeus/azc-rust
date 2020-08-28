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
    let source_file_result = source_file::load_from_string(
        r#"
            module: iam
            symbols:
              - identifier: User
                kind: entity
                parameters:
                  description: "Test yo!"
              - identifier: Application
                kind: entity
              - identifier: Terminal
                kind: adjunct
                parameters:
                  entities:
                    - Application
            "#,
    );
    if source_file_result.is_ok() {
        println!("Success! {:?}", source_file_result.unwrap());
    } else {
        println!("Error? {:?}", source_file_result.err());
    }

    let source_file_result = source_file::load_from_file("testdata/iam/user.yaml");
    match source_file_result {
        Ok(x) => println!("Success2! {:?}", x),
        Err(y) => println!("Error2! {:?}", y),
    }
}
