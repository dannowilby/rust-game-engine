
use glfw::{ Context };

pub fn create_window(name: &str, sx: u32, sy: u32) -> (glfw::Window, std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>, glfw::Glfw) {
  
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));

  let (mut window, events) = glfw.create_window(sx, sy, name, glfw::WindowMode::Windowed).unwrap();
  
  window.make_current();
  window.set_key_polling(true);
  window.set_size_polling(true);
  window.set_cursor_pos_polling(true);

  gl::load_with(|s| window.get_proc_address(s) as *const _);
  gl::Viewport::load_with(|s| window.get_proc_address(s) as *const _);

  (window, events, glfw)
}

pub fn has_resized(window: &glfw::Window, sx: i32, sy: i32) -> bool {

  let (nx, ny) = window.get_size();

  !((nx == sx) && (ny == sy))
}

pub fn print_opengl_version() {
  unsafe {
    let mut x = 0;
    let mut y = 0;
    gl::GetIntegerv(gl::MAJOR_VERSION, &mut x);
    gl::GetIntegerv(gl::MINOR_VERSION, &mut y);
    println!("OpenGL Version {}.{}", x, y);
  }
}