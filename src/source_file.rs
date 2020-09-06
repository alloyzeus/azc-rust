//

use std::{convert::TryFrom, fs::File, io::BufReader, path::Path};

use crate::{azyaml, base::result, source_file_serde, symbol};

#[derive(Debug)]
pub struct SourceFile {
    pub module: String,

    pub symbols: Vec<symbol::Symbol>,
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> result::Result<SourceFile> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let sf: source_file_serde::SourceFileSerde = azyaml::from_reader(reader)?;
    let sf = SourceFile::try_from(sf)?;
    Ok(sf)
}
