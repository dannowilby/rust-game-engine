
use crate::state::{ GameState };

use glfw::{Action, Context, Key};

pub struct Engine {
  states: Vec<GameState>,
  current_state: usize,
}

impl Engine {
  
  pub fn new() -> Engine {
    Engine {
      states: Vec::new(),
      current_state: 0,
    }
  }

  pub fn change_state(&mut self, new_state: usize) {
    self.current_state = new_state;
  }

  pub fn add_state(&mut self, state: GameState) {
    self.states.push(state);
  }

}

pub fn default_events(window: &mut glfw::Window, event: &glfw::WindowEvent) {

  match event {
    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
      window.set_should_close(true)
    },
    _ => {},
  }
}

pub fn start_engine(engine: &mut Engine, glfw: &mut glfw::Glfw, window: &mut glfw::Window, events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>) {

  let current_state = &mut engine.states[engine.current_state];
  current_state.init();

  while !window.should_close() {
    
    window.swap_buffers();
    glfw.poll_events();

    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); }

    // systems execute
    current_state.systems.execute(&window, &mut current_state.data);

    for (_, event) in glfw::flush_messages(&events) {
      default_events(window, &event);
    }
  }
 
  current_state.destroy();
  
}
