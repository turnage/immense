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

mod builtin;
mod transforms;

pub use self::builtin::*;
pub use self::transforms::*;

use auto_from::auto_from;
use crate::mesh::{Mesh, PrimitiveMesh, Vertex};
use palette::rgb::Rgb;
use std::rc::Rc;

/// A composition of subrules to expand until meshes are generated.
#[derive(Clone)]
pub struct Rule {
    invocations: Vec<(Option<Transform>, RuleInternal)>,
}

/// An ergonomics macro for defining rules out of transformed subrule invocations.
///
/// Where normally you would have to write
///
/// ````
/// # use immense::*;
/// let rule = Rule::new().push(Tf::tx(2.0), cube())
///                       .push(Tf::s(0.5), cube());
/// ````
///
/// you can write
///
/// ````
/// # use immense::*;
/// let rule = rule![Tf::tx(2.0) => cube(),
///                  Tf::s(0.5) => cube()];
/// ````
#[macro_export]
macro_rules! rule {
    ($($transforms:expr => $subrule:expr),+ $(,)*) => ({
        let mut rule = Rule::new();
        $(let rule = rule.push($transforms, $subrule);)*
        rule
    });
}

impl Rule {
    /// Returns a new rule that contains no subrules.
    pub fn new() -> Rule {
        Rule {
            invocations: vec![],
        }
    }

    pub(crate) fn primitive(mesh: PrimitiveMesh) -> Self {
        let mut rule = Rule::new();
        rule.invocations
            .push((None, RuleInternal::Mesh(OutputMeshSource::Primitive(mesh))));
        rule
    }

    pub(crate) fn mesh(mesh: Rc<Mesh>) -> Self {
        let mut rule = Rule::new();
        rule.invocations
            .push((None, RuleInternal::Mesh(OutputMeshSource::Dynamic(mesh))));
        rule
    }

    /// Adds a subrule to the Rule.
    pub fn push(mut self, transforms: impl Into<TransformArgument>, rule: impl ToRule) -> Rule {
        match transforms.into() {
            TransformArgument::Single(transform) => {
                self.invocations
                    .push((Some(transform), RuleInternal::Invocations(Rc::new(rule))));
            }
            TransformArgument::Many(ref transforms) if !transforms.is_empty() => {
                let rule = Rc::new(rule);
                self.invocations.append(
                    &mut transforms
                        .into_iter()
                        .map(|transform| {
                            (Some(*transform), RuleInternal::Invocations(rule.clone()))
                        })
                        .collect(),
                );
            }

            _ => self
                .invocations
                .push((None, RuleInternal::Invocations(Rc::new(rule)))),
        };
        self
    }

    /// Returns an iterator expands the Rule's subrules, outputting the meshes it generates until
    /// all rules have been fully expanded. As an iterator the meshes are computed lazily so you can
    /// use this method and terminate with [take][std::iter::Iterator::take], or
    /// [until][std::iter::Iterator::take_while], etc if your rule tree is infinite.
    pub fn generate(self) -> impl Iterator<Item = OutputMesh> {
        let root = RuleInternal::Invocations(Rc::new(self));
        MeshIter::new(vec![(None, root)])
    }
}

/// An iterator that iterates over a [Rule][self::Rule]'s generated meshes.
pub struct MeshIter {
    rules: Vec<(Option<Transform>, RuleInternal)>,
}

impl MeshIter {
    fn new(rules: Vec<(Option<Transform>, RuleInternal)>) -> Self {
        Self { rules }
    }
}

/// An OutputMesh can be written out in an object file.
#[derive(Debug)]
pub struct OutputMesh {
    transform: Option<Transform>,
    source: OutputMeshSource,
}

#[derive(Debug, Clone)]
enum OutputMeshSource {
    Primitive(PrimitiveMesh),
    Dynamic(Rc<Mesh>),
}

impl OutputMesh {
    pub(crate) fn color(&self) -> Rgb {
        self.transform.unwrap_or(Transform::default()).get_color()
    }

    /// An iterator over the vertices that compose the mesh. Access `.x`, `.y`, and `.z`.
    pub fn vertices<'a>(&'a self) -> impl Iterator<Item = Vertex> + 'a {
        self.mesh()
            .vertices()
            .iter()
            .map(move |v: &'a Vertex| -> Vertex {
                self.transform.map(|t| t.apply_to(*v)).unwrap_or(*v)
            })
    }

    /// An iterator over the normals of each vertex if they are defined for the mesh.
    pub fn normals<'a>(&'a self) -> Option<impl Iterator<Item = Vertex> + 'a> {
        match self.mesh().normals() {
            Some(ref normals) => Some(Box::new(normals.iter().map(move |v: &Vertex| -> Vertex {
                self.transform.map(|t| t.apply_to(*v)).unwrap_or(*v)
            }))),
            None => None,
        }
    }

    /// An iterator over the faces of the output mesh.
    ///
    /// Important things to note if you are not writing out an object file:
    ///
    /// * These faces are not necessarily triangles.
    /// * The vertex indices start at 1.
    pub fn faces<'a>(&'a self) -> impl Iterator<Item = &'a [usize]> {
        self.mesh().faces()
    }

    pub(crate) fn mesh<'a>(&'a self) -> &'a Mesh {
        match self.source {
            OutputMeshSource::Primitive(ref primitive) => primitive.mesh(),
            OutputMeshSource::Dynamic(ref mesh) => mesh.as_ref(),
        }
    }
}

impl Iterator for MeshIter {
    type Item = OutputMesh;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((transform, rule)) = self.rules.pop() {
            match rule {
                RuleInternal::Mesh(mesh) => {
                    return Some(OutputMesh {
                        transform,
                        source: mesh,
                    })
                }
                RuleInternal::Invocations(composite_rule) => {
                    let composite_rule = composite_rule.to_rule();
                    self.rules.reserve(composite_rule.invocations.len());
                    for (sub_transform, sub_rule) in composite_rule.invocations {
                        self.rules.push((
                            match (transform, sub_transform) {
                                (None, None) => None,
                                (Some(parent), None) => Some(parent),
                                (Some(parent), Some(child)) => Some(parent.cons(child)),
                                (None, Some(child)) => Some(child),
                            },
                            sub_rule,
                        ));
                    }
                }
            }
        }
        None
    }
}

/// A trait for types that can become rules.
pub trait ToRule: 'static {
    fn to_rule(&self) -> Rule;
}

impl ToRule for Rule {
    fn to_rule(&self) -> Rule {
        self.clone()
    }
}

impl ToRule for Rc<Mesh> {
    fn to_rule(&self) -> Rule {
        Rule::mesh(self.clone())
    }
}

#[auto_from]
#[derive(Clone)]
enum RuleInternal {
    Mesh(OutputMeshSource),
    Invocations(Rc<ToRule>),
}
