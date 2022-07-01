//

use std::{collections::HashMap, convert};

use super::{
    error, generator, generator_go, generator_go_yaml, source_file, symbol, symbol_yaml, yaml,
};

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
        let mut options: HashMap<String, Box<dyn generator::GeneratorOptions>> = HashMap::new();
        for o in x.options {
            match o.key.as_str() {
                "go" => {
                    let def: generator_go_yaml::GeneratorGoOptionsYaml =
                        yaml::from_value(o.value.clone())?;
                    options.insert(
                        o.key,
                        Box::new(generator_go::GeneratorGoOptions::try_from(def)?),
                    );
                }
                _ => (), //TODO: error
            }
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
    value: yaml::Value,
}
