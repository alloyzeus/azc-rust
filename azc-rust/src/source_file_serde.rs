//

use crate::{source_file, symbol, symbol_serde};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SourceFileSerde {
    module: String,

    #[serde(default)]
    symbols: Vec<symbol_serde::SymbolSerde>,
}

impl Into<source_file::SourceFile> for SourceFileSerde {
    fn into(self) -> source_file::SourceFile {
        source_file::SourceFile {
            module: self.module,
            symbols: self
                .symbols
                .into_iter()
                .map(|x| symbol::Symbol::from(x.into()))
                .collect(),
        }
    }
}
