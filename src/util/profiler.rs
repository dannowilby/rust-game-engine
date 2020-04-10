
use std::time::{ SystemTime };

pub struct Profiler {
  start_time: SystemTime,
  end_time: SystemTime,
  pub delta: f32,
}

impl Profiler {
  
  pub fn new() -> Profiler {
    Profiler { 
      start_time: SystemTime::now(),
      end_time: SystemTime::now(),
      delta: 0f32,
    }
  }

  pub fn start(&mut self) {
    self.start_time = SystemTime::now();
  }

  pub fn end(&mut self) {
    self.end_time = SystemTime::now();

    self.delta = (self.end_time.duration_since(self.start_time).unwrap().as_micros() as f32) / (1000000 as f32);
  }
}