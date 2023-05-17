use std::fmt::{Formatter, Debug, Result};

use bevy_ecs::prelude::Entity;

pub enum Error {
    NoRootEntity,
    EntityNotFound(Entity),
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::NoRootEntity => write!(f, "No root entity"),
            Error::EntityNotFound(e) => write!(f, "Entity not found: {:?}", e),
        }
    }
}