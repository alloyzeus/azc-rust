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
    generator_options: Vec<SourceFileGeneratorOptions>,
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
        for o in x.generator_options {
            match o.generator.as_str() {
                "go" => {
                    let def: generator_go_yaml::GeneratorGoOptionsYaml =
                        yaml::from_value(o.options.clone())?;
                    options.insert(
                        o.generator,
                        Box::new(generator_go::GeneratorGoOptions::try_from(def)?),
                    );
                }
                _ => (), //TODO: error
            }
        }
        Ok(Self {
            module: x.module,
            symbols: symbols,
            generator_options: options,
        })
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct SourceFileGeneratorOptions {
    generator: String,
    options: yaml::Value,
}
