
use std::time::{ SystemTime };

pub struct Profiler {
  start_time: SystemTime,
  name: String,
}

impl Profiler {
  
  pub fn new(process: &str) -> Profiler {
    Profiler { 
      start_time: SystemTime::now(),
      name: String::from(process),
    }
  }

  pub fn start(&mut self) {
    self.start_time = SystemTime::now();
  }

  pub fn end(&mut self) {
    match self.start_time.elapsed() {
      Ok(elapsed) => {
        println!("\"{}\" took {}ms! or {}ns!", self.name, elapsed.as_millis(), elapsed.as_nanos());
      },
      Err(e) => {
        println!("{}", e);
      }
    }
  }
}