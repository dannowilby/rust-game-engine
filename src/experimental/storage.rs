use std::collections::{ BTreeMap };
use std::any::{ Any };
use std::ptr;
use std::mem;

pub const MAX_ENTITIES: usize = 1000;

macro_rules! make_array {
  ($n:expr, $constructor:expr) => {{
    unsafe {
      let mut items: [_; $n] = mem::MaybeUninit::uninit().assume_init();
      for (i, place) in items.iter_mut().enumerate() {
          ptr::write(place, $constructor(i));
      }
      items
    }
  }}
}

pub trait Component: Sized + Any {
    type Storage: Storage<Self>;
}

pub trait Storage<T>: Any {
    fn new() -> Self
    where
        Self: Sized;

    fn set(&mut self, index: usize, value: T);
}

pub struct ArrayStorage<T: 'static> {
  internal: [Option<T>; MAX_ENTITIES],
}

impl<T> Storage<T> for ArrayStorage<T> {
  
  fn new() -> Self {
    Self {
      internal: make_array!(MAX_ENTITIES, |_i| { Option::<T>::None }),
    }
  }

  fn set(&mut self, index: usize, value: T) {
    self.internal[index] = Some(value);
  }
}

pub struct ComponentManager {
    storages: BTreeMap<String, Box<dyn Any>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            storages: BTreeMap::new(),
        }
    }

    pub fn get_storage_mut<C: Component>(&mut self, storage: &str) -> &mut <C as Component>::Storage {

        if !self.storages.contains_key(storage) {
            let new_storage = <C as Component>::Storage::new();

            self.storages.insert(String::from(storage), Box::new(new_storage));
        }

        match self.storages.get_mut(storage) {
            Some(probably_storage) => {
                match probably_storage.downcast_mut::<<C as Component>::Storage>() {
                    Some(storage) => storage,
                    None => unreachable!(), // <- you may want to do something less explosive here
                }
            }
            None => unreachable!(),
        }
    }

    pub fn get_storage<C: Component>(&self, storage: &str) -> Option<&<C as Component>::Storage> {
        match self.storages.get(storage) {
            Some(probably_storage) => {
                match probably_storage.downcast_ref::<<C as Component>::Storage>() {
                    Some(storage) => Some(storage),
                    None => unreachable!(), // <- you may want to do something less explosive here
                }
            }
            None => None,
        }
    }
}

// struct Vec2(f64, f64);
// impl Component for Vec2 { type Storage = ArrayStorage<Self>; }

pub fn test() {
  // let mut c_mgr: ComponentManager = ComponentManager::new();
  
  /*
  let mut storage = (c_mgr).get_storage_mut::<Vec2i>("position");

  let mut counter = 0;
  for n in 0..MAX_ENTITIES {
    storage.set(n, Vec2i(0 + counter, 0 + counter));
    counter = counter + 1;
  }

  let store = c_mgr.get_storage::<Vec2i>("position").unwrap();

  println!("size {}", mem::size_of::<[Option<RenderMesh>; MAX_ENTITIES]>());

  for n in 0..MAX_ENTITIES {
    if let Some(i) = &store.internal[n] {
      
      // bind and render the vaos or whatev
      let Vec2i(x, y) = i;

      println!("entity: {}, x: {}, y: {}, x + y: {}", n, x, y, x + y);
    }
  }
  */
}
