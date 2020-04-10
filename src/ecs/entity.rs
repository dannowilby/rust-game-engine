
use crate::component::{ ComponentManager };

use std::collections::HashSet;

pub struct EntityManager {
  pub entities: HashSet<Entity>,
}

pub type Entity = u32;

impl EntityManager {

  pub fn new() -> EntityManager {
    EntityManager { entities: HashSet::new() }
  }

  pub fn create_entity(&mut self, entity: Entity) -> Entity {
    //self.entities.insert(entity);
    entity
  }

  // does not remove entity from vector yet--is TODO
  pub fn kill_entity(&mut self, component_manager: &mut ComponentManager, entity: Entity) {

    //component_manager.delete_entity_entries(entity);
    //self.entities.remove(&entity);
  }
}

// systems
