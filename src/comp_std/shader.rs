
use std::fs;
use std::ffi::{ CString };
use std::ptr;
use std::str;

use nalgebra::{ Matrix4 };

use crate::component::{ Component, ArrayStorage };

pub struct Shader {
  pub id: u32,
}
impl Component for Shader { type Storage = ArrayStorage<Self>; }

pub fn use_shader(shader: u32) {
  unsafe {
    gl::UseProgram(shader);
  }
}

pub fn set_int_uniform(shader: u32, name: &str, data: u32) {

  unsafe {
    
    let location = gl::GetUniformLocation(shader, CString::new(name.as_bytes()).unwrap().as_ptr());
  
    gl::Uniform1i(location, data as i32);
  }
}

pub fn set_matrix_uniform(shader: u32, name: &str, matrix: &Matrix4<f32>) {

  unsafe {
    
    let location = gl::GetUniformLocation(shader, CString::new(name.as_bytes()).unwrap().as_ptr());
  
    gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
  }
}

#[allow(unused_assignments)]
pub fn create_shader(vertex: &str, fragment: &str) -> u32 {

  let mut shader = 0u32;

  let vertex_source = fs::read_to_string(vertex).expect("Unable to read file.");
  let fragment_source = fs::read_to_string(fragment).expect("Unable to read file.");

  let vertex_shader_code = CString::new(vertex_source.as_bytes()).unwrap();
  let fragment_shader_code = CString::new(fragment_source.as_bytes()).unwrap();

  unsafe {

    let vertex = gl::CreateShader(gl::VERTEX_SHADER);
    gl::ShaderSource(vertex, 1, &vertex_shader_code.as_ptr(), ptr::null());
    gl::CompileShader(vertex);
    
    let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
    gl::ShaderSource(fragment, 1, &fragment_shader_code.as_ptr(), ptr::null());
    gl::CompileShader(fragment);
    
    let id = gl::CreateProgram();
    gl::AttachShader(id, vertex);
    gl::AttachShader(id, fragment);
    gl::LinkProgram(id);
    
    gl::DeleteShader(vertex);
    gl::DeleteShader(fragment);
    shader = id;
  }

  shader
}