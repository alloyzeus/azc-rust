//

use crate::azml::{entity::entity_id, oid};

pub type EntityIdInteger = oid::IntegerId;

impl entity_id::EntityIdDefinition for EntityIdInteger {}

pub trait EntityIdIntegerEncoding: mopa::Any + std::fmt::Debug {}

mopafy!(EntityIdIntegerEncoding);
