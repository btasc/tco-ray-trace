pub struct Model {
    name: Option<String>,
    verticies: Vec<f32>,
    triangles: Vec<[u32, 3]>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            name: None,
            verticies: Vec::new(),
            triangles: Vec::new(),
        }
    }

    pub fn from_obj(obj_file: String) -> Self {
        todo!()
    }
}