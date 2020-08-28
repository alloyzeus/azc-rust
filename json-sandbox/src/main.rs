extern crate serde;

use serde::{Serialize, Deserialize};
use serde_yaml::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    #[serde(rename="display_name")]
    name: String,

    age: u8,

    phones: Vec<String>,
}

fn load_yaml() -> Result<String> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "display_name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_yaml::from_str(data)?;

    // Do things just like with any other Rust data structure.
    Ok(format!("Please call {} at the number {}", p.name, p.phones[0]))
}

fn main() {
    let contact = load_yaml();
    if contact.is_ok() {
        println!("Success! {:?}", contact.unwrap());
    } else {
        println!("Error? {:?}", contact.err());
    }
}
