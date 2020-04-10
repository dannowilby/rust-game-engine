
extern crate glfw;
extern crate image;
extern crate tobj;

// ecs

#[path = "ecs/entity.rs"]
pub mod entity;

#[path = "ecs/component.rs"]
pub mod component;

#[path = "ecs/system.rs"]
pub mod system;


// comp_std

#[path = "comp_std/mesh.rs"]
pub mod mesh;

#[path = "comp_std/model.rs"]
pub mod model;

#[path = "comp_std/shader.rs"]
pub mod shader;

#[path = "comp_std/framebuffer.rs"]
pub mod framebuffer;


// util

#[path = "util/profiler.rs"]
pub mod profiler;


// engine

#[path = "engine/display.rs"]
pub mod display;

#[path = "engine/engine.rs"]
pub mod engine;

#[path = "engine/state.rs"]
pub mod state;

#[path = "engine/player.rs"]
pub mod player;

#[path = "engine/lua.rs"]
pub mod lua;


// states

//pub mod animal;
//pub mod mine;
#[path = "test/test.rs"]
pub mod test;

use crate::display::{ create_window };
use crate::engine::{ Engine, start_engine };

use crate::profiler::{ Profiler };

use crate::state::{ GameState };
use crate::test::{ init_test_state, destroy_test_state };

// as of now there is only a client,
// server-side code has not been implemented
// but support for it is still planned for in
// the future.
fn main() {

  let (mut window, events, mut glfw) = create_window("Bashura", 800, 400);
  
  let mut profiler = Profiler::new();
  profiler.start();

  let mut engine = Engine::new();

  engine.add_state(GameState::new(init_test_state, destroy_test_state));

  profiler.end();
  println!("\"bootup\" took {}s", profiler.delta);

  start_engine(&mut engine, &mut glfw, &mut window, events);
}