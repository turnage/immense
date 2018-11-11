use crate::parameters::Parameters;
use nalgebra::Matrix4x1;

pub type Vertex = Matrix4x1<f32>;

pub fn vertex(x: f32, y: f32, z: f32) -> Vertex {
    Vertex::new(x, y, z, 1.0)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Vec<usize>>,
}

impl Mesh {
    pub fn apply_parameters(mut self, parameters: Parameters) -> Self {
        for vert in &mut self.vertices {
            *vert = parameters.transform * (*vert);
        }
        self
    }
}