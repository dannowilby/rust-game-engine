
pub struct Framebuffer {
  vao: u32,
  texture: u32,
  start_index: u32,
  end_index: u32, 
  render_systems: Vec<fn()>
}