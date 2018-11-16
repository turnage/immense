use crate::Tf;
use genmesh::generators::{IcoSphere, IndexedPolygon, SharedVertex};
use lazy_static::lazy_static;
use nalgebra::Matrix4x1;

pub type Vertex = Matrix4x1<f32>;

fn vertex(x: f32, y: f32, z: f32) -> Vertex {
    Vertex::new(x, y, z, 1.0)
}

lazy_static! {
    static ref CUBE_MESH: Mesh = Mesh::new(
        vec![
            vertex(-0.5, 0.5, 0.5),
            vertex(-0.5, -0.5, 0.5),
            vertex(0.5, -0.5, 0.5),
            vertex(0.5, 0.5, 0.5),
            vertex(-0.5, 0.5, -0.5),
            vertex(-0.5, -0.5, -0.5),
            vertex(0.5, -0.5, -0.5),
            vertex(0.5, 0.5, -0.5),
        ],
        None,
        vec![
            vec![1, 2, 3, 4],
            vec![8, 7, 6, 5],
            vec![4, 3, 7, 8],
            vec![5, 1, 4, 8],
            vec![5, 6, 2, 1],
            vec![2, 6, 7, 3],
        ]
    );
    static ref ICO_SPHERE: Mesh = Mesh::new(
        IcoSphere::new()
            .shared_vertex_iter()
            .map(|v| Tf::s(0.5).apply_to(vertex(v.pos.x, v.pos.y, v.pos.z)))
            .collect(),
        Some(
            IcoSphere::new()
                .shared_vertex_iter()
                .map(|v| vertex(v.normal.x, v.normal.y, v.normal.z))
                .collect()
        ),
        IcoSphere::new()
            .indexed_polygon_iter()
            .map(|t| vec![t.x + 1, t.y + 1, t.z + 1])
            .collect()
    );
}

pub(crate) struct Mesh {
    vertices: Vec<Vertex>,
    normals: Option<Vec<Vertex>>,
    faces: Vec<Vec<usize>>,
}

impl Mesh {
    fn new(vertices: Vec<Vertex>, normals: Option<Vec<Vertex>>, faces: Vec<Vec<usize>>) -> Self {
        Self {
            vertices,
            normals,
            faces: faces,
        }
    }

    pub(crate) fn vertices<'a>(&'a self) -> &'a [Vertex] {
        self.vertices.as_slice()
    }

    pub(crate) fn normals<'a>(&'a self) -> Option<&'a [Vertex]> {
        self.normals.as_ref().map(|ns| ns.as_slice())
    }

    pub(crate) fn faces<'a>(&'a self) -> impl Iterator<Item = &'a [usize]> {
        self.faces.iter().map(|f| f.as_slice())
    }
}

#[derive(Clone, Debug)]
pub enum PrimitiveMesh {
    Cube,
    IcoSphere,
}

impl PrimitiveMesh {
    pub(crate) fn mesh(&self) -> &'static Mesh {
        match *self {
            PrimitiveMesh::Cube => &*CUBE_MESH,
            PrimitiveMesh::IcoSphere => &*ICO_SPHERE,
        }
    }
}
