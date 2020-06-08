
use crate::entity::{ EntityManager };
use crate::component::{ ComponentManager };
use crate::system::{ SystemManager };

use crate::player::{ Player };

use nalgebra::{ Matrix4 };

use rlua::{ Lua };

pub struct GameState {

  init: fn(y: &mut Self),
  destroy: fn(y: &mut Self),

  pub logic_systems: SystemManager<GameStateData>,
  pub render_systems: SystemManager<GameStateData>,

  pub data: GameStateData,
}

pub struct GameStateData {

  pub entities: EntityManager,
  pub components: ComponentManager,

  pub player: Player,

  pub lua: Lua,
}

impl GameState {

  pub fn new(init: fn(y: &mut Self), destroy: fn(y: &mut Self)) -> GameState {
    GameState {
      
      logic_systems: SystemManager::new(),
      render_systems: SystemManager::new(),

      data: GameStateData {

        entities: EntityManager::new(),
        components: ComponentManager::new(),

        player: Player {
          projection: Matrix4::identity(),
          size: (0, 0),
          camera: Matrix4::identity(),
        },

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