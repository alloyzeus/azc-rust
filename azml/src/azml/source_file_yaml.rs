//

use std::{collections::HashMap, convert};

use super::{error, source_file, symbol, symbol_yaml};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SourceFileYaml {
    module: String,

    #[serde(default)]
    symbols: Vec<symbol_yaml::SymbolYaml>,

    #[serde(default)]
    options: Vec<SourceFileOption>,
}

impl convert::TryFrom<SourceFileYaml> for source_file::SourceFile {
    type Error = error::Error;

    fn try_from(x: SourceFileYaml) -> Result<Self, Self::Error> {
        let symbols = x
            .symbols
            .iter()
            .map(|x| symbol::Symbol::try_from(x))
            .collect::<Result<Vec<symbol::Symbol>, _>>()?;
        let mut options = HashMap::new();
        for o in x.options {
            options.insert(o.key, o.value);
        }
        Ok(source_file::SourceFile {
            module: x.module,
            symbols: symbols,
            options: options,
        })
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct SourceFileOption {
    key: String,
    value: String,
}
