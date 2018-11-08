use nalgebra::Vector3;

pub type Vertex = Vector3<f32>;

#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Vec<usize>>,
}
