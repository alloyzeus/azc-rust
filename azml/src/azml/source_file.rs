//

use std::{collections::HashMap, convert::TryFrom, fs, io, path};

use super::{generator, result, source_file_yaml, symbol, yaml};

#[derive(Debug)]
pub struct SourceFile {
    pub module: String,

    pub symbols: Vec<symbol::Symbol>,

    pub options: HashMap<String, Box<dyn generator::GeneratorOptions>>,
}

pub fn load_from_file<P: AsRef<path::Path>>(path: P) -> result::Result<SourceFile> {
    // Open the file in read-only mode with buffer.
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    let sf: source_file_yaml::SourceFileYaml = yaml::from_reader(reader)?;
    let sf = SourceFile::try_from(sf)?;
    Ok(sf)
}
