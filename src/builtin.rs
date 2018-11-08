use crate::mesh::{Mesh, Vertex};
use crate::system::{Character, Parameters, Rule};
use lazy_static::lazy_static;

lazy_static! {
    static ref CUBE_MESH: Mesh = Mesh {
        vertices: vec![
            Vertex::new(0.0, 1.0, 0.0),
            Vertex::new(0.0, 0.0, 1.0),
            Vertex::new(1.0, 0.0, 1.0),
            Vertex::new(1.0, 1.0, 1.0),
            Vertex::new(0.0, 1.0, 0.0),
            Vertex::new(0.0, 0.0, 0.0),
            Vertex::new(1.0, 0.0, 0.0),
            Vertex::new(1.0, 1.0, 0.0)
        ],
        faces: vec![
            vec![1, 2, 3, 4],
            vec![8, 7, 6, 5],
            vec![4, 3, 3, 8],
            vec![5, 1, 4, 8],
            vec![5, 6, 2, 1],
            vec![2, 6, 7, 3],
        ]
    };
}

fn cube(parameters: Parameters) -> Mesh {
    CUBE_MESH.clone()
}
