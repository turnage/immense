mod builtin;
mod transforms;

pub use self::builtin::*;
pub use self::transforms::*;

use auto_from::auto_from;
use crate::mesh::Mesh;
use rayon::prelude::*;
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

    /// Iteratively expands the Rule's subrules until meshes are generated.
    pub fn generate(self) -> Vec<Mesh> {
        let root = RuleInternal::Invocations(Arc::new(self));
        let mut meshes: Vec<(Option<Transform>, Mesh)> = vec![];
        let mut rules: Vec<(Option<Transform>, RuleInternal)> = vec![(None, root)];
        while let Some((transform, rule)) = rules.pop() {
            match rule {
                RuleInternal::Mesh(mesh) => meshes.push((transform, mesh.clone())),
                RuleInternal::Invocations(composite_rule) => {
                    let composite_rule = composite_rule.to_rule();
                    rules.reserve(composite_rule.invocations.len());
                    for (sub_transform, sub_rule) in composite_rule.invocations {
                        rules.push((
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
        meshes
            .into_par_iter()
            .map(|(transform, mesh)| match transform {
                Some(transform) => transform.apply_to(mesh),
                None => mesh,
            })
            .collect()
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
