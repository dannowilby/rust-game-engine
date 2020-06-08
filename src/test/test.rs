
use crate::state::{ GameState, GameStateData };
use crate::lua::{ load_lua_file };
use glfw::{Action, Key};

use crate::shader::{ Shader, create_shader };
use crate::mesh::{ BasicMesh, build_basic_mesh, render_basic_mesh_system };
use crate::component::{ Storage };

use crate::player::{ update_projection, default_camera_controls };

use nalgebra::{ Vector4, Matrix4 };

pub fn init_test_state(gs: &mut GameState) {

  unsafe { 
    gl::Enable(gl::CULL_FACE);
    gl::Enable(gl::DEPTH_TEST); 
  }
  
  // Components
  {
    let storage = gs.data.components.get_storage_mut::<BasicMesh>("mesh");
    let mesh = build_basic_mesh(vec![
      0.0,  0.0, -1.0,
      0.5,  0.0, -1.0,
      0.0,  0.5, -1.0,
    ]);
    storage.set(0, mesh);
  }
  {
    let storage = gs.data.components.get_storage_mut::<Matrix4<f32>>("transformation");
    storage.set(0, Matrix4::identity());
  }
  {
    let storage = gs.data.components.get_storage_mut::<Shader>("shader");
    storage.set(0, Shader { id: create_shader("shaders/vertex.sf", "shaders/fragment.sf") });
  }
  
  // Systems

  gs.render_systems.push(render_basic_mesh_system);

  gs.logic_systems.push(update_projection);
  gs.systems.push(default_camera_controls);

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
