//

use crate::{source_file_serde, symbol};
use serde_yaml::Result;

#[derive(Debug)]
pub struct SourceFile {
    pub module: String,

    pub symbols: Vec<symbol::Symbol>,
}

pub fn load_from_string(data: &str) -> Result<SourceFile> {
    let p: source_file_serde::SourceFileSerde = serde_yaml::from_str(data)?;
    Ok(p.into())
}
