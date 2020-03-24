
use crate::state::{ GameStateData };

use glfw::{Action, Key};
use nalgebra::{ Matrix4, Vector3 };

pub struct Player {
  
  pub projection: Matrix4<f32>,
  pub size: (i32, i32),

  pub camera: Matrix4<f32>,
}

pub fn update_projection(player: &mut Player, window: &glfw::Window) {

  let (cx, cy) = player.size;
  let (sx, sy) = window.get_size();
  
  if cx != sx || cy != sy {

    player.projection = Matrix4::new_perspective((sx as f32)/(sy as f32), 1.309, 0.1, 100.0);
    unsafe { gl::Viewport(0, 0, sx, sy); }

    player.size = window.get_size();
  }
}

pub fn default_camera_controls(window: &glfw::Window, state: &mut GameStateData) {

  if window.get_key(Key::W) == Action::Press {
    state.player.camera.append_translation_mut(&Vector3::new(0.0, 0.0, 0.1));
  }
  if window.get_key(Key::S) == Action::Press {
    state.player.camera.append_translation_mut(&Vector3::new(0.0, 0.0, -0.1));
  }

  if window.get_key(Key::A) == Action::Press {
    state.player.camera.append_translation_mut(&Vector3::new(0.1, 0.0, 0.0));
  }
  
  if window.get_key(Key::D) == Action::Press {
    state.player.camera.append_translation_mut(&Vector3::new(-0.1, 0.0, 0.0));
  }

}