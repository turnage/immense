// Copyright 2018 The immense Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::Tf;
use genmesh::generators::{IcoSphere, IndexedPolygon, SharedVertex};
use lazy_static::lazy_static;
use nalgebra::Matrix4x1;
use std::rc::Rc;

pub type Vertex = Matrix4x1<f32>;

/// Initializes a vertex for a custom mesh.
pub fn vertex(x: f32, y: f32, z: f32) -> Vertex {
    Vertex::new(x, y, z, 1.0)
}

pub(crate) fn sphere_of_resolution(resolution: usize) -> Mesh {
    Mesh::new(
        IcoSphere::subdivide(resolution)
            .shared_vertex_iter()
            .map(|v| Tf::s(0.5).apply_to(vertex(v.pos.x, v.pos.y, v.pos.z)))
            .collect(),
        Some(
            IcoSphere::subdivide(resolution)
                .shared_vertex_iter()
                .map(|v| vertex(v.normal.x, v.normal.y, v.normal.z))
                .collect(),
        ),
        IcoSphere::subdivide(resolution)
            .indexed_polygon_iter()
            .map(|t| vec![t.x + 1, t.y + 1, t.z + 1])
            .collect(),
    )
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
    static ref ICO_SPHERE: Mesh = sphere_of_resolution(0);
}

/// A custom mesh definition described by a set of vertices, normals, and faces.
///
/// This is a low-level type and you are expected to know what you are doing in this part of the API.
///     1. There should be a normal for each vertex.
///     2. Each face is a set of indices to the vertices that the face connects.
///     3. Vertex indices start at 1, according to the object file standard.
#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    normals: Option<Vec<Vertex>>,
    faces: Vec<Vec<usize>>,
}

impl Mesh {
    /// Allocates a mesh from the given vertices, normals, and faces, which can invoked as rules.
    pub fn from(
        vertices: Vec<Vertex>,
        normals: Option<Vec<Vertex>>,
        faces: Vec<Vec<usize>>,
    ) -> Rc<Self> {
        Rc::new(Self::new(vertices, normals, faces))
    }

    pub(crate) fn new(
        vertices: Vec<Vertex>,
        normals: Option<Vec<Vertex>>,
        faces: Vec<Vec<usize>>,
    ) -> Self {
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
