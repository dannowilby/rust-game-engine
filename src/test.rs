
use crate::state::{ GameState, GameStateData };
use crate::lua::{ load_lua_file };
use glfw::{Action, Key};

use crate::shader::{ create_shader };
use crate::mesh::{ build_vao, render_mesh_system };

use nalgebra::{ Vector4 };

fn test_system(window: &glfw::Window, state: &mut GameStateData) {

  if window.get_key(Key::W) == Action::Press {
    
    let t1 = state.components.get_vec3f("position", 0);
    state.components.set_vec3f("position", 0, [ t1[0] + 0.1, t1[1] + 0.1, t1[2] + 0.1 ]);
    
    let t2 = state.components.get_vec3f("position", 0);

    println!("x: {} y: {} z: {}", t2[0], t2[1], t2[2]);
  }

}

// load the test state, build the 
// meshes, load the lua files, add
// the system functions, etc...
pub fn init_test_state(gs: &mut GameState) {

  gs.data.components.register("mesh");
  gs.data.components.register("position");

  let e1 = gs.data.entities.create_entity(0);

  let (vao, len) = build_vao(vec![
    0.5, -0.5, 0.0,
    -0.5, -0.5, 0.0,
    0.0,  0.5, 0.0,
  ]);
  let shader = create_shader("shaders/vertex.sf", "shaders/fragment.sf");

  gs.data.components.set_vec4u("mesh", e1, Vector4::new(vao, shader, 0u32, len as u32));

  gs.systems.push(render_mesh_system);
  gs.systems.push(test_system);

  gs.data.lua.context(|_lua_ctx| {
    
    load_lua_file(&_lua_ctx, "lua/test.lua");
    
    //_lua_ctx.load("test()").eval::<()>();

  });

}

// manually garabage collect
// all the GL data that is usual
pub fn destroy_test_state(gs: &mut GameState) {
  
  gs.data.lua.context(|_lua_ctx| {
    //_lua_ctx.load("test()").eval::<()>();
  });

}
