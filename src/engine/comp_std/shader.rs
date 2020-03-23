
use std::fs;
use std::ffi::{ CString };
use std::ptr;
use std::str;

pub fn use_shader(shader: u32) {
  unsafe {
    gl::UseProgram(shader);
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