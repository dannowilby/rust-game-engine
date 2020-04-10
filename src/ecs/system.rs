
pub struct SystemManager<T> { 
  systems: Vec<System<T>>
}

impl<T> SystemManager<T> {

  pub fn new() -> SystemManager<T> {
    SystemManager { systems: Vec::new() }
  }

  pub fn push(&mut self, sys: System<T>) {
    self.systems.push(sys);
  }

  pub fn execute(&self, window: &glfw::Window, state: &mut T, delta: f32) {

    for function in &self.systems {
      function(window, state, delta);
    }
  }
}

pub type System<T> = fn(window: &glfw::Window, state: &mut T, delta: f32);

