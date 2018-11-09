use crate::mesh::{vertex, Mesh, Vertex};
use crate::parameters::Parameters;
use crate::system::{Character, Producer};
use lazy_static::lazy_static;

lazy_static! {
    static ref CUBE_MESH: Mesh = Mesh {
        vertices: vec![
            vertex(-0.5, 0.5, 0.5),
            vertex(-0.5, -0.5, 0.5),
            vertex(0.5, -0.5, 0.5),
            vertex(0.5, 0.5, 0.5),
            vertex(-0.5, 0.5, -0.5),
            vertex(-0.5, -0.5, -0.5),
            vertex(0.5, -0.5, -0.5),
            vertex(0.5, 0.5, -0.5)
        ],
        faces: vec![
            vec![1, 2, 3, 4],
            vec![8, 7, 6, 5],
            vec![4, 3, 7, 8],
            vec![5, 1, 4, 8],
            vec![5, 6, 2, 1],
            vec![2, 6, 7, 3],
        ]
    };
}

pub struct Cube;

impl Producer for Cube {
    fn produce(&self, parameters: Parameters) -> Vec<Character> {
        vec![Character::Constant(
            CUBE_MESH.clone().apply_parameters(&parameters),
        )]
    }
}

fn cube(parameters: Parameters) -> Mesh {
    CUBE_MESH.clone()
}
