mod builtin;
mod transforms;

pub use self::builtin::*;
pub use self::transforms::*;

use auto_from::auto_from;
use crate::mesh::Mesh;
use std::sync::Arc;

/// A composition of subrules to expand until meshes are generated.
#[derive(Clone)]
pub struct Rule {
    invocations: Vec<(Option<Transform>, RuleInternal)>,
}

impl Rule {
    /// Returns a new rule that contains no subrules.
    pub fn new() -> Rule {
        Rule {
            invocations: vec![],
        }
    }

    pub(crate) fn push_mesh(mut self, mesh: Mesh) -> Self {
        self.invocations.push((None, RuleInternal::Mesh(mesh)));
        self
    }

    /// Adds a subrule to the Rule.
    pub fn push(
        mut self,
        transforms: impl Into<TransformArgument>,
        rule: impl ToRule + 'static,
    ) -> Rule {
        match transforms.into() {
            TransformArgument::Single(transform) => {
                self.invocations
                    .push((Some(transform), RuleInternal::Invocations(Arc::new(rule))));
            }
            TransformArgument::Many(transforms) if !transforms.is_empty() => {
                let rule = Arc::new(rule);
                self.invocations.append(
                    &mut transforms
                        .into_iter()
                        .map(|transform| (Some(transform), RuleInternal::Invocations(rule.clone())))
                        .collect(),
                );
            }

            _ => self
                .invocations
                .push((None, RuleInternal::Invocations(Arc::new(rule)))),
        };
        self
    }

    /// Returns an iterator expands the Rule's subrules, outputting the meshes it generates until
    /// all rules have been fully expanded. As an iterator the meshes are computed lazily so you can
    /// use this method and terminate with [take][std::iter::Iterator::take], or
    /// [until][std::iter::Iterator::until], etc if your rule tree is infinite.
    pub fn generate(self) -> impl Iterator<Item = OutputMesh> {
        let root = RuleInternal::Invocations(Arc::new(self));
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
    pub transform: Option<Transform>,
    pub mesh: Mesh,
}

impl Iterator for MeshIter {
    type Item = OutputMesh;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((transform, rule)) = self.rules.pop() {
            match rule {
                RuleInternal::Mesh(mesh) => return Some(OutputMesh { transform, mesh }),
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
pub trait ToRule: Send + Sync {
    fn to_rule(&self) -> Rule;
}

impl ToRule for Rule {
    fn to_rule(&self) -> Rule {
        self.clone()
    }
}

#[auto_from]
#[derive(Clone)]
enum RuleInternal {
    Mesh(Mesh),
    Invocations(Arc<ToRule>),
}
