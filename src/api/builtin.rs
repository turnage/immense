use crate::api::Rule;
use crate::mesh::Mesh;

/// A cube of size 1 whose center is at the origin.
pub fn cube() -> Rule {
    Rule::new().push_mesh(Mesh::Cube)
}
