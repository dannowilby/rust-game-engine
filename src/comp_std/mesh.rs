
use crate::shader::{ Shader, use_shader, set_matrix_uniform, set_int_uniform };
use crate::state::{ GameStateData };
use crate::component::{ Component, ArrayStorage, MAX_ENTITIES };

use std::ptr;
use std::mem;
use gl;
use gl::types::*;
use std::os::raw::c_void;

use nalgebra::{ Vector3, Vector4, Matrix4 };

use std::path::Path;

use image::GenericImage;

pub struct BasicMesh {
  vao: u32,
  start_index: i32,
  end_index: i32,
}
impl Component for BasicMesh { type Storage = ArrayStorage<Self>; }

impl Component for Matrix4<f32> { type Storage = ArrayStorage<Self>; }

pub fn build_basic_mesh(vertices: Vec<f32>) -> BasicMesh {
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

  BasicMesh { vao: vao, start_index: 0, end_index: (vertices.len() as i32) }
}


#[allow(unused_variables)]
pub fn render_basic_mesh_system(window: &glfw::Window, state: &mut GameStateData, delta: f32) {
  
  let meshes = state.components.get_storage::<BasicMesh>("mesh").unwrap();
  let transformations = state.components.get_storage::<Matrix4<f32>>("transformation").unwrap();
  let shaders = state.components.get_storage::<Shader>("shader").unwrap();

  for n in 0..MAX_ENTITIES {

    if let Some(mesh) = &meshes.internal[n] {
      if let Some(transformation) = &transformations.internal[n] {
        if let Some(shader) = &shaders.internal[n] {

          use_shader(shader.id);

          set_matrix_uniform(shader.id, "projection", &state.player.projection);
          set_matrix_uniform(shader.id, "view", &state.player.camera);
          set_matrix_uniform(shader.id, "model", &transformation);

          unsafe {
            gl::BindVertexArray(mesh.vao);
            gl::DrawArrays(gl::TRIANGLES, mesh.start_index, mesh.end_index);
          }

        }
      }
    }

  }
}