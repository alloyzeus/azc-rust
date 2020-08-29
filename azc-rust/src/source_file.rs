//

use crate::{result, source_file_serde, symbol};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
pub struct SourceFile {
    pub module: String,

    pub symbols: Vec<symbol::Symbol>,
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> result::Result<SourceFile> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u: source_file_serde::SourceFileSerde = serde_yaml::from_reader(reader)?;
    Ok(u.into())
}
