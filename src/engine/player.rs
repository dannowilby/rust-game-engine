
use crate::state::GameState;

use nalgebra::{ Matrix4 };

pub struct Player {
  
  pub projection: Matrix4<f32>,

  pub camera: Matrix4<f32>,

}

pub fn default_camera_controls(state: &mut GameState) {

}