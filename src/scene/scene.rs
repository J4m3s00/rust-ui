use bevy_ecs::{world::World, prelude::{Component, Entity}, schedule::Schedule};

use crate::error::Error;

use super::entity_tree::EntityTree;

#[derive(Component)]
pub struct EntityBase {
    pub name: String,
}

pub struct Scene {
    entities: EntityTree,
    root_entity: Entity,
    world: World,
}

impl Scene {
    pub fn new() -> Self {
        let mut world = World::default();
        let root_entity = world.spawn(EntityBase {
            name: "root".to_string(),
        }).id();
        Self {
            entities: EntityTree::new(root_entity),
            root_entity,
            world,
        }
    }

    // Creates a new empty entity. Can be placed at optional parent
    pub fn add_entity(&mut self, parent: Option<Entity>) -> Result<Entity, Error> {
        let entity = self.world.spawn(EntityBase {
            name: "entity".to_string(),
        }).id();
        self.entities.add_child(parent.unwrap_or(self.root_entity), entity)?;
        Ok(entity)
    }

    pub fn remove_entity(&mut self, entity: Entity) -> Result<(), Error> {
        self.entities.remove(entity)?;
        self.world.despawn(entity);
        Ok(())
    }

    pub fn add_component<T>(&mut self, entity: Entity) {
    }

    // Schedules
    pub fn run_schedule(&mut self, schedule: &mut Schedule) {
        schedule.run(&mut self.world);
    }

}
