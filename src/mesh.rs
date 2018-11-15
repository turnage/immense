use core::array::FixedSizeArray;
use lazy_static::lazy_static;
use nalgebra::Matrix4x1;

pub type Vertex = Matrix4x1<f32>;

fn vertex(x: f32, y: f32, z: f32) -> Vertex {
    Vertex::new(x, y, z, 1.0)
}

lazy_static! {
    static ref CUBE_VERTICES: [Vertex; 8] = [
        vertex(-0.5, 0.5, 0.5),
        vertex(-0.5, -0.5, 0.5),
        vertex(0.5, -0.5, 0.5),
        vertex(0.5, 0.5, 0.5),
        vertex(-0.5, 0.5, -0.5),
        vertex(-0.5, -0.5, -0.5),
        vertex(0.5, -0.5, -0.5),
        vertex(0.5, 0.5, -0.5),
    ];
    static ref CUBE_FACES: &'static [&'static [usize]] = &[
        &[1, 2, 3, 4],
        &[8, 7, 6, 5],
        &[4, 3, 7, 8],
        &[5, 1, 4, 8],
        &[5, 6, 2, 1],
        &[2, 6, 7, 3],
    ];
}

#[derive(Clone, Debug)]
pub enum Mesh {
    Cube,
}

impl Mesh {
    pub fn vertices<'a>(&'a self) -> &'a [Vertex] {
        match *self {
            Mesh::Cube => CUBE_VERTICES.as_slice(),
        }
    }

    pub fn faces<'a>(&'a self) -> &'a [&'a [usize]] {
        match *self {
            Mesh::Cube => *CUBE_FACES,
        }
    }
}
