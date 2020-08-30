//

use crate::{adjunct, arity_serde};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AdjunctSerde {
    #[serde(default)]
    kind: String,

    hosts: Vec<AdjuctEntitySerde>,

    #[serde(default)]
    arity: arity_serde::ArityConstraintSerde,
}

impl From<AdjunctSerde> for adjunct::Adjunct {
    fn from(x: AdjunctSerde) -> adjunct::Adjunct {
        match x.kind.as_str() {
            "entity" => adjunct::Adjunct {
                kind: adjunct::AdjunctKind::Entity,
                hosts: x.hosts.into_iter().map(|x| x.into()).collect(),
                arity: x.arity.into(),
            },
            _ => adjunct::Adjunct {
                kind: adjunct::AdjunctKind::ValueObject,
                hosts: x.hosts.into_iter().map(|x| x.into()).collect(),
                arity: x.arity.into(),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AdjuctEntitySerde {
    name: String,
}

impl From<AdjuctEntitySerde> for adjunct::AdjuctHost {
    fn from(x: AdjuctEntitySerde) -> adjunct::AdjuctHost {
        adjunct::AdjuctHost { name: x.name }
    }
}
