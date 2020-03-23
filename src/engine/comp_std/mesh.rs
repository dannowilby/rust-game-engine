
use crate::state::{ GameStateData };

use crate::shader::{ use_shader };

use std::ptr;
use std::mem;
use gl;
use gl::types::*;
use std::os::raw::c_void;

pub fn build_vao(vertices: Vec<f32>) -> (u32, usize) {
  let (mut vbo, mut vao) = (0, 0);

  unsafe {

    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    
    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
      gl::ARRAY_BUFFER,
      (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
      &vertices[0] as *const f32 as *const c_void,
      gl::STATIC_DRAW
    );

    let stride = 3 * mem::size_of::<GLfloat>() as GLsizei;
    
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);
  }

  (vao, vertices.len())
}

#[allow(unused_variables)]
pub fn render_mesh_system(window: &glfw::Window, state: &mut GameStateData) {
  
  for e in &state.entities.entities {
    
    let v = state.components.get_vec4u("mesh", *e);
    
    use_shader(v.y);

    unsafe {
      gl::BindVertexArray(v.x);
      gl::DrawArrays(gl::TRIANGLES, v.z as i32, v.w as i32);
    }
  }
}