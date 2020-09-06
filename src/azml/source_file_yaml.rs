//

use std::convert;

use crate::azml::{error, source_file, symbol, symbol_yaml};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SourceFileYaml {
    module: String,

    #[serde(default)]
    symbols: Vec<symbol_yaml::SymbolYaml>,
}

impl convert::TryFrom<SourceFileYaml> for source_file::SourceFile {
    type Error = error::Error;

    fn try_from(x: SourceFileYaml) -> Result<Self, Self::Error> {
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
