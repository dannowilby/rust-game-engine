/*
pub fn load_model(shader: u32, model_location: &str, texture_location: &str) -> (u32, usize, u32) {
  
  let mut vao = 0;
  let mut vbo = 0;
  let mut ebo = 0;
  let mut texture = 0;

  let model = tobj::load_obj(&Path::new(model_location));
  let (models, materials) = model.unwrap();

  let img = image::open(&Path::new(texture_location)).unwrap();

  // DEBUG - print model data
  println!("# of models: {}", models.len());
  println!("# of materials: {}", materials.len());
  println!("pos = {}, normals = {}, uv = {}", models[0].mesh.positions.len(), models[0].mesh.normals.len(), models[0].mesh.texcoords.len());
  println!("mat = {}", materials.len());

  let mesh = &models[0].mesh;
  let num_vertices = mesh.positions.len() / 3;
  
  let mut vertices_data: Vec<f32> = Vec::new();
  let indices_data: Vec<u32> = mesh.indices.clone();

  for i in 0..num_vertices {
    vertices_data.push(mesh.positions[i*3]);
    vertices_data.push(mesh.positions[i*3 + 1]);
    vertices_data.push(mesh.positions[i*3 + 2]);

    vertices_data.push(mesh.texcoords[i*2]);
    vertices_data.push(mesh.texcoords[i*2 + 1]);
  }

  unsafe {

    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    gl::GenBuffers(1, &mut ebo);
    gl::GenTextures(1, &mut texture);

    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
      gl::ARRAY_BUFFER,
      (vertices_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
      &vertices_data[0] as *const f32 as *const c_void,
      gl::STATIC_DRAW
    );

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(
      gl::ELEMENT_ARRAY_BUFFER, 
      (indices_data.len() * mem::size_of::<u32>()) as GLsizeiptr,
      &indices_data[0] as *const u32 as *const c_void, 
      gl::STATIC_DRAW
    );

    let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
    
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);

    gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(1);

    let img = img.flipv();
    let n_img = img.to_rgba();
    let height = n_img.height();
    let width = n_img.width();
    let texture_data = (n_img).into_raw();

    println!("w={} h={}", width, height);
    println!("data={}", texture_data.len());

    gl::BindTexture(gl::TEXTURE_2D, texture);
    gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width as i32, height as i32,
        0, gl::RGBA, gl::UNSIGNED_BYTE, &texture_data[0] as *const u8 as *const c_void);
    gl::GenerateMipmap(gl::TEXTURE_2D);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    gl::BindVertexArray(0);
  }

  (vao, indices_data.len(), texture)
}
*/

//#[allow(unused_variables)]
//pub fn render_model_system(window: &glfw::Window, state: &mut GameStateData, delta: f32) {

  /*for e in &state.entities.entities {
    
    if state.components.has_entity_in_storage("model", *e) {

      let v = state.components.get_vec4u("model", *e);
      
      unsafe { gl::BindTexture(gl::TEXTURE_2D, v.w); }

      use_shader(v.x);
      
      let mut m = Matrix4::identity();
      m.append_translation_mut(&Vector3::new(8.0 * *e as f32, 0.0, 0.0));

      set_matrix_uniform(v.x, "projection", &state.player.projection);
      set_matrix_uniform(v.x, "view", &state.player.camera);
      set_matrix_uniform(v.x, "model", &m);

      unsafe {
        gl::BindVertexArray(v.y);
        gl::DrawElements(gl::TRIANGLES, v.z as i32, gl::UNSIGNED_INT, ptr::null());
      }
    }
  }*/
//}