mod builtin;
mod transforms;

pub use self::builtin::*;
pub use self::transforms::*;

use crate::mesh::Mesh;
use crate::parameters::Parameters;
use itertools::Either;
use nalgebra::Matrix4;
use std::rc::Rc;

#[derive(Clone)]
pub struct Rule {
    transforms: Vec<Matrix4<f32>>,
    inner: RuleInner,
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            transforms: vec![identity()],
            inner: RuleInner::default(),
        }
    }
}

pub trait RuleBuilder {
    fn build_rule(&self, rule: Rule) -> Rule;
}

impl RuleBuilder for Rule {
    fn build_rule(&self, rule: Rule) -> Rule {
        self.clone()
    }
}

#[derive(Clone)]
enum RuleInner {
    Invocations(Vec<Rc<RuleBuilder>>),
    Mesh(Mesh),
}

impl Default for RuleInner {
    fn default() -> Self {
        RuleInner::Invocations(vec![])
    }
}

struct Invocation {
    parameters: Parameters,
    rule: Rule,
}

impl Rule {
    pub(crate) fn mesh(mesh: Mesh) -> Self {
        Self {
            inner: RuleInner::Mesh(mesh),
            ..Default::default()
        }
    }

    pub fn tf(self, tf: impl Transform) -> Self {
        Self {
            transforms: {
                let mut transforms = vec![];
                for prefix in tf.transform() {
                    println!("applying {:?}", prefix);
                    for suffix in &self.transforms {
                        transforms.push(prefix * (*suffix));
                    }
                }
                println!("transforms: {:?}", transforms);
                transforms
            },
            ..self
        }
    }

    pub fn push(self, rule: impl RuleBuilder + 'static) -> Self {
        if let RuleInner::Invocations(mut invocations) = self.inner {
            Self {
                inner: RuleInner::Invocations({
                    invocations.push(Rc::new(rule));
                    invocations
                }),
                ..self
            }
        } else {
            self
        }
    }

    pub fn build(self, parameters: Parameters) -> Vec<Mesh> {
        Rule::compile(vec![], vec![], parameters, self)
    }

    fn expand(self, parameters: Parameters) -> Either<Vec<Mesh>, Vec<Invocation>> {
        let parameters: Vec<Parameters> = self.transforms.iter()
                            .map(|t| Parameters {
                                transform: parameters.transform * (*t),
                                ..parameters
                            }).collect();
        match self.inner {
            RuleInner::Invocations(invocations) => Either::Right(
                invocations
                    .into_iter()
                    .flat_map(|rule_builder| {
                        let rule = rule_builder.build_rule(Rule::default());
                        parameters.iter().map(|parameters| Invocation {
                                parameters: *parameters,
                                rule: rule.clone(),
                            })
                            .collect::<Vec<Invocation>>()
                    })
                    .collect(),
            ),
            RuleInner::Mesh(mesh) => Either::Left(parameters.iter().map(|parameters| mesh.clone().apply_parameters(*parameters)).collect()),
        }
    }

    fn compile(
        mut meshes: Vec<Mesh>,
        mut invocations: Vec<Invocation>,
        parameters: Parameters,
        rule: Rule,
    ) -> Vec<Mesh> {
        match rule.expand(parameters) {
            Either::Left(mut new_meshes) => {
                meshes.append(&mut new_meshes);
            }
            Either::Right(mut next_invocations) => {
                invocations.append(&mut next_invocations);
            }
        };

        match invocations.pop() {
            Some(invocation) => {
                Rule::compile(meshes, invocations, invocation.parameters, invocation.rule)
            }
            None => meshes,
        }
    }
}
