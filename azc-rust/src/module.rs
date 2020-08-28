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

trait SymbolParameters: 'static + std::fmt::Debug {}

#[derive(Debug)]
struct Entity {
    description: String,
    service: Option<EntityService>,
}

impl SymbolParameters for Entity {}

#[derive(Debug)]
struct EntityService {
    description: String,
}

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
        let params: Option<EntitySerde>;
        if self.parameters.is_mapping() {
            params = serde_yaml::from_value(self.parameters).unwrap();
        } else {
            params = None;
        }
        Symbol {
            name: self.name,
            kind: self.kind,
            parameters: if params.is_some() {
                Some(Box::new(Entity::from(params.unwrap().into())))
            } else {
                None
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
struct EntitySerde {
    #[serde(default)]
    description: String,

    #[serde(default)]
    service: Option<EntityServiceSerde>,
}

impl Into<Entity> for EntitySerde {
    fn into(self) -> Entity {
        println!("self {:?}", self.service);
        Entity {
            description: self.description,
            service: if self.service.is_some() {
                Some(self.service.unwrap().into())
            } else {
                None
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct EntityServiceSerde {
    description: String,
}

impl Into<EntityService> for EntityServiceSerde {
    fn into(self) -> EntityService {
        EntityService {
            description: self.description,
        }
    }
}

pub fn load_source_file() -> Result<SourceFile> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        module: iam
        symbols:
          - name: User
            kind: entity
            parameters:
              description: "Test yo!"
          - name: Application
            kind: entity
        "#;

    let p: SourceFileSerde = serde_yaml::from_str(data)?;
    Ok(p.into())
}
