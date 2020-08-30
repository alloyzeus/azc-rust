//

use crate::{source_file, symbol_serde};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SourceFileSerde {
    module: String,

    #[serde(default)]
    symbols: Vec<symbol_serde::SymbolSerde>,
}

impl From<SourceFileSerde> for source_file::SourceFile {
    fn from(x: SourceFileSerde) -> source_file::SourceFile {
        source_file::SourceFile {
            module: x.module,
            symbols: x.symbols.into_iter().map(|x| x.into()).collect(),
        }
    }
}
