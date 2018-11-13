mod builtin;
mod transforms;

pub use self::builtin::*;
pub use self::transforms::*;

use auto_from::auto_from;
use crate::mesh::Mesh;
use rayon::prelude::*;
use std::rc::Rc;

/// A composition of subrules to expand until meshes are generated.
#[derive(Clone, Debug)]
pub struct Rule {
    invocations: Vec<(Option<Transform>, RuleRef)>,
}

impl Rule {
    pub fn new() -> Rule {
        Rule {
            invocations: vec![],
        }
    }

    pub fn push(mut self, transforms: impl Into<TransformInput>, rule_ref: RuleRef) -> Rule {
        match transforms.into() {
            TransformInput::Single(transform) => {
                self.invocations.push((Some(transform), rule_ref));
            }
            TransformInput::Many(transforms) if !transforms.is_empty() => {
                self.invocations.append(
                    &mut transforms
                        .into_iter()
                        .map(|transform| (Some(transform), rule_ref))
                        .collect(),
                );
            }

            _ => self.invocations.push((None, rule_ref)),
        };
        self
    }
}

pub trait ToRule {
    fn to_rule(&self) -> Rule;
}

impl ToRule for Rule {
    fn to_rule(&self) -> Rule {
        self.clone()
    }
}

impl ToRule for RuleRef {
    fn to_rule(&self) -> Rule {
        Rule::new().push(None, *self)
    }
}

#[auto_from]
#[derive(Clone)]
enum RuleInternal {
    Mesh(Mesh),
    Invocations(Rc<ToRule>),
}

#[derive(Copy, Clone, Debug)]
pub struct RuleRef {
    rule: usize,
}

pub struct System {
    rules: Vec<RuleInternal>,
}

impl System {
    const RULE_CUBE: RuleRef = RuleRef { rule: 0 };

    pub fn new() -> Self {
        Self {
            rules: vec![cube_mesh().into()],
        }
    }

    pub fn define(&mut self, rule: impl ToRule + 'static) -> RuleRef {
        self.rules.push(RuleInternal::Invocations(Rc::new(rule)));
        RuleRef {
            rule: self.rules.len() - 1,
        }
    }

    pub fn generate(&self, root: impl ToRule + 'static) -> Vec<Mesh> {
        let root = RuleInternal::Invocations(Rc::new(root));
        let mut meshes: Vec<(Option<Transform>, Mesh)> = vec![];
        let mut rules: Vec<(Option<Transform>, &RuleInternal)> = vec![(None, &root)];
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
                            &self.rules[sub_rule.rule],
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
