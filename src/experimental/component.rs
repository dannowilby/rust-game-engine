
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::{ Vector2, Vector3, Vector4 };
use crate::entity::{ Entity };

pub struct ComponentManager {
  component_storages: HashMap<String, RefCell<ComponentStorage>>,
}

pub type ComponentStorage = HashMap<Entity, Component>;

pub enum Component {
  
  Vec3f([f32; 3]),
  Vecfsize(Rc<Vec<f32>>),

  Int(i32),
  Uint(u32),

  Vec2u(Vector2<u32>),
  Vec3u(Vector3<u32>),
  Vec4u(Vector4<u32>),

  String(String),
  
  None(),
}

impl ComponentManager {

  pub fn new() -> ComponentManager {
    ComponentManager { component_storages: HashMap::new() }
  }

  pub fn register(&mut self, name: &str) {

    let storage = HashMap::new();

    self.component_storages.insert(
      String::from(name),
      RefCell::new(storage),
    );
  }

  pub fn delete_entity_entries(&mut self, entity: Entity) {

    for k in self.component_storages.values() {
      
      let mut c = k.borrow_mut();
      
      if c.contains_key(&entity) {
        c.remove(&entity);
      }
    }
  }

  pub fn has_storage(&self, name: &str) -> bool {
    self.component_storages.contains_key(name)
  }

  pub fn has_entity_in_storage(&self, name: &str, uid: Entity) -> bool {
    self.has_storage(name) && self.component_storages[name].borrow().contains_key(&uid)
  }

  // TYPE METHODS

  // Vec3f
  
  pub fn set_vec3f(&mut self, storage: &str, uid: Entity, data: [f32; 3]) {
    
    if !self.has_storage(storage) {
      println!("Storage error");
      return;
    }

    self.component_storages[storage].borrow_mut().insert(uid, Component::Vec3f(data));
  }

  pub fn get_vec3f(&self, storage: &str, uid: Entity) -> [f32; 3] {
    
    if !self.has_entity_in_storage(storage, uid) {
      println!("Entity Storage error");
      return [ 0.0, 0.0, 0.0 ];
    }
    
    match self.component_storages[storage].borrow()[&uid] {
      Component::Vec3f(data) => data,
      _ => [0.0, 0.0, 0.0],
    }
  }

  // Vecfsize

  pub fn set_vecfsize(&mut self, storage: &str, uid: Entity, data: Vec<f32>) {
    
    if !self.has_storage(storage) {
      println!("Storage error");
      return;
    }

    self.component_storages[storage].borrow_mut().insert(uid, Component::Vecfsize(Rc::new(data)));
  }

  pub fn get_vecfsize(&self, storage: &str, uid: Entity) -> Rc<Vec<f32>> {
    
    if !self.has_entity_in_storage(storage, uid) {
      println!("Entity Storage error");
      return Rc::new(vec![ 0.0 ]);
    }

    match &self.component_storages[storage].borrow()[&uid] {
      Component::Vecfsize(data) => Rc::clone(&data),
      _ => Rc::new(vec![ 0.0 ]),
    }
  }

  // Vector2 u32

  pub fn set_vec2u(&mut self, storage: &str, uid: Entity, data: Vector2<u32>) {
    
    if !self.has_storage(storage) {
      println!("Storage error");
      return;
    }

    self.component_storages[storage].borrow_mut().insert(uid, Component::Vec2u(data));
  }

  pub fn get_vec2u(&self, storage: &str, uid: Entity) -> Vector2<u32> {
    
    if !self.has_entity_in_storage(storage, uid) {
      println!("Entity Storage error");
      return Vector2::new(0u32, 0u32);
    }

    match &self.component_storages[storage].borrow()[&uid] {
      Component::Vec2u(data) => *(data),
      _ => Vector2::new(0u32, 0u32),
    }
  }

  // Vector3 u32

  pub fn set_vec3u(&mut self, storage: &str, uid: Entity, data: Vector3<u32>) {
    
    if !self.has_storage(storage) {
      println!("Storage error");
      return;
    }

    self.component_storages[storage].borrow_mut().insert(uid, Component::Vec3u(data));
  }

  pub fn get_vec3u(&self, storage: &str, uid: Entity) -> Vector3<u32> {
    
    if !self.has_entity_in_storage(storage, uid) {
      println!("Entity Storage error");
      return Vector3::new(0u32, 0u32, 0u32);
    }

    match &self.component_storages[storage].borrow()[&uid] {
      Component::Vec3u(data) => *(data),
      _ => Vector3::new(0u32, 0u32, 0u32),
    }
  }

  // Vector4 u32

  pub fn set_vec4u(&mut self, storage: &str, uid: Entity, data: Vector4<u32>) {
    
    if !self.has_storage(storage) {
      println!("Storage error");
      return;
    }

    self.component_storages[storage].borrow_mut().insert(uid, Component::Vec4u(data));
  }

  pub fn get_vec4u(&self, storage: &str, uid: Entity) -> Vector4<u32> {
    
    if !self.has_entity_in_storage(storage, uid) {
      println!("Entity Storage error: {}", uid);
      return Vector4::new(0u32, 0u32, 0u32, 0u32);
    }

    match &self.component_storages[storage].borrow()[&uid] {
      Component::Vec4u(data) => *(data),
      _ => Vector4::new(0u32, 0u32, 0u32, 0u32),
    }
  }
}