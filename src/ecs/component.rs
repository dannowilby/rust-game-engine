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
  pub internal: [Option<T>; MAX_ENTITIES],
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
                    None => unreachable!(),
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
                    None => unreachable!(),
                }
            }
            None => None,
        }
    }
}
