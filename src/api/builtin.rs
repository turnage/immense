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
