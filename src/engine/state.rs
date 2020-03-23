
use crate::entity::{ EntityManager };
use crate::component::{ ComponentManager };
use crate::system::{ SystemManager };

use rlua::{ Lua };

pub struct GameState {

  init: fn(y: &mut Self),
  destroy: fn(y: &mut Self),

  pub systems: SystemManager<GameStateData>,

  pub data: GameStateData,
}

pub struct GameStateData {

  pub entities: EntityManager,
  pub components: ComponentManager,

  pub lua: Lua,
}

impl GameState {

  pub fn new(init: fn(y: &mut Self), destroy: fn(y: &mut Self)) -> GameState {
    GameState {
      
      systems: SystemManager::new(),

      data: GameStateData {

        entities: EntityManager::new(),
        components: ComponentManager::new(),  

        lua: Lua::new(),
      },

      init: init,
      destroy: destroy,
    }
  }

  pub fn init(&mut self) {
    (self.init)(self);
  }

  pub fn destroy(&mut self) {
    (self.destroy)(self);
  }
}