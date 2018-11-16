use crate::api::Rule;
use crate::mesh::{sphere_of_resolution, Mesh, PrimitiveMesh};
use std::rc::Rc;

/// A cube of size 1 whose center is at the origin.
pub fn cube() -> Rule {
    Rule::primitive(PrimitiveMesh::Cube)
}

/// An icosphere of diameter 1.
pub fn icosphere() -> Rule {
    Rule::primitive(PrimitiveMesh::IcoSphere)
}

/// A sphere of the given resolution. Produces 20 * 4 ^ resolution polygons to estimate the sphere.
///
/// This is an expensive mesh. Try to call this function once and use the Rc wherever needed.
pub fn sphere(resolution: usize) -> Rc<Mesh> {
    Rc::new(sphere_of_resolution(resolution))
}
