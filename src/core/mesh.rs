use crate::core::component::Component;
use crate::math::vector_3f::Vector3F;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: (f32, f32, f32)
}

implement_vertex!(Vertex, position);

#[derive(Copy, Clone, Debug)]
pub struct Normal {
    pub normal: (f32, f32, f32)
}

implement_vertex!(Normal, normal);


#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u16>,
    pub face_count: u32,
    pub size: Vector3F,
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            vertices: Vec::new(),
            normals: Vec::new(),
            face_count: 0,
            size: Vector3F::zero(),
            indices: Vec::new()
        }
    }
}

impl Component for Mesh {

}
