
use crate::state::{ GameStateData };

use glfw::{Action, Key};
use nalgebra::{ Matrix4, Vector3 };

pub struct Player {
  
  pub projection: Matrix4<f32>,
  pub size: (i32, i32),

  pub camera: Matrix4<f32>,
}

pub fn update_projection(window: &glfw::Window, state: &mut GameStateData, delta: f32) {

  let (cx, cy) = state.player.size;
  let (sx, sy) = window.get_size();
  
  if cx != sx || cy != sy {

    state.player.projection = Matrix4::new_perspective((sx as f32)/(sy as f32), 1.309, 0.1, 100.0);
    unsafe { gl::Viewport(0, 0, sx, sy); }

    state.player.size = window.get_size();
  }
}

pub fn default_camera_controls(window: &glfw::Window, state: &mut GameStateData, delta: f32) {

  let speed = (6f32 * delta);

  if window.get_key(Key::W) == Action::Press {
    state.player.camera.append_translation_mut(&Vector3::new(0.0, 0.0, speed));
  }
  if window.get_key(Key::S) == Action::Press {
    state.player.camera.append_translation_mut(&Vector3::new(0.0, 0.0, -speed));
  }

  if window.get_key(Key::A) == Action::Press {
    state.player.camera.append_translation_mut(&Vector3::new(speed, 0.0, 0.0));
  }
  
  if window.get_key(Key::D) == Action::Press {
    state.player.camera.append_translation_mut(&Vector3::new(-speed, 0.0, 0.0));
  }

  if window.get_key(Key::T) == Action::Press {
    state.player.camera.append_translation_mut(&Vector3::new(0.0, -speed, 0.0));
  }

  if window.get_key(Key::G) == Action::Press {
    state.player.camera.append_translation_mut(&Vector3::new(0.0, speed, 0.0));
  }

  if window.get_key(Key::I) == Action::Press {
    let rot        = Matrix4::from_scaled_axis(&Vector3::x() * 3.1415 * delta);

    state.player.camera = state.player.camera * rot;
  }

  if window.get_key(Key::K) == Action::Press {
    let rot        = Matrix4::from_scaled_axis(&Vector3::x() * -3.1415 * delta);

    state.player.camera = state.player.camera * rot;
  }

  if window.get_key(Key::N) == Action::Press {
    unsafe {
      gl::PolygonMode(gl::FRONT, gl::LINE);
      gl::PolygonMode(gl::BACK, gl::LINE);
    }
  }

  if window.get_key(Key::M) == Action::Press {
    unsafe {
      gl::PolygonMode(gl::FRONT, gl::FILL);
      gl::PolygonMode(gl::BACK, gl::FILL);
    }
  }

}