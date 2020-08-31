//

use serde::{Deserialize, Serialize};
use std::convert;

use crate::{base::error, source_file, symbol, symbol_serde};

#[derive(Serialize, Deserialize)]
pub struct SourceFileSerde {
    module: String,

    #[serde(default)]
    symbols: Vec<symbol_serde::SymbolSerde>,
}

impl convert::TryFrom<SourceFileSerde> for source_file::SourceFile {
    type Error = error::Error;

    fn try_from(x: SourceFileSerde) -> Result<Self, Self::Error> {
        let symbols = x
            .symbols
            .into_iter()
            .map(|x| symbol::Symbol::try_from(x))
            .collect::<Result<Vec<symbol::Symbol>, _>>()?;
        Ok(source_file::SourceFile {
            module: x.module,
            symbols: symbols,
        })
    }
}
