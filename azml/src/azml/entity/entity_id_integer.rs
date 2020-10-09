//

use crate::azml::{eid, entity::entity_id};

pub type EntityIdInteger = eid::IntegerId;

impl entity_id::EntityIdDefinition for EntityIdInteger {}

pub trait EntityIdIntegerEncoding: mopa::Any + std::fmt::Debug {}

mopafy!(EntityIdIntegerEncoding);
