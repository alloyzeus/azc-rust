//

use crate::entity;
use crate::entity_serde;
use serde::{Deserialize, Serialize};
use serde_yaml::Result;

#[derive(Debug)]
pub struct SourceFile {
    module: String,

    symbols: Vec<Symbol>,
}

#[derive(Debug)]
struct Symbol {
    name: String,
    kind: String,
    parameters: Option<Box<dyn SymbolParameters + 'static>>,
}

pub trait SymbolParameters: 'static + std::fmt::Debug {}

// Serde stuff

#[derive(Serialize, Deserialize)]
struct SourceFileSerde {
    module: String,

    #[serde(default)]
    symbols: Vec<SymbolSerde>,
}

impl Into<SourceFile> for SourceFileSerde {
    fn into(self) -> SourceFile {
        SourceFile {
            module: self.module,
            symbols: self
                .symbols
                .into_iter()
                .map(|x| Symbol::from(x.into()))
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SymbolSerde {
    name: String,
    kind: String,

    #[serde(default)]
    parameters: serde_yaml::Value,
}

impl Into<Symbol> for SymbolSerde {
    fn into(self) -> Symbol {
        let params: Option<entity_serde::EntitySerde>;
        if self.parameters.is_mapping() {
            params = serde_yaml::from_value(self.parameters).unwrap();
        } else {
            params = None;
        }
        Symbol {
            name: self.name,
            kind: self.kind,
            parameters: if params.is_some() {
                Some(Box::new(entity::Entity::from(params.unwrap().into())))
            } else {
                None
            },
        }
    }
}

pub fn load_source_file_from_string(data: &str) -> Result<SourceFile> {
    let p: SourceFileSerde = serde_yaml::from_str(data)?;
    Ok(p.into())
}
