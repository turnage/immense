use crate::api::Rule;
use crate::mesh::PrimitiveMesh;

/// A cube of size 1 whose center is at the origin.
pub fn cube() -> Rule {
    Rule::new().primitive(PrimitiveMesh::Cube)
}

/// An icosphere of diameter 1.
pub fn icosphere() -> Rule {
    Rule::new().primitive(PrimitiveMesh::IcoSphere)
}
