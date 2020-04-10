
use rlua::{ Context, Chunk };

use std::fs;

pub fn get_file_contents(file: &str) -> String {
  fs::read_to_string(file).expect("Unable to read file.")
}

pub fn load_lua_file(context: &Context, file: &str) {
  
  let code = get_file_contents(file);

  let c: Chunk = context.load(&code);

  c.exec().unwrap();
}
