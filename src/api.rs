mod builtin;
mod transforms;

pub use self::builtin::*;
pub use self::transforms::*;

use crate::mesh::Mesh;
use crate::parameters::Parameters;
use itertools::Either;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct Rule {
    inner: RuleInner,
}

#[derive(Clone)]
enum RuleInner {
    Invocations(Vec<InvocationBuilder>),
    Mesh(Mesh),
}

#[derive(Clone)]
struct InvocationBuilder {
    transforms: Vec<Rc<dyn Transform>>,
    rule: Rule,
}

impl InvocationBuilder {
    fn build(self, parameters: Parameters) -> Vec<Invocation> {
        let mut invocations = vec![];
        for transform in self.transforms {
            for parameters in transform.transform(parameters) {
                invocations.push(Invocation {
                    parameters,
                    rule: self.rule.clone(),
                });
            }
        }
        invocations
    }
}

struct Invocation {
    parameters: Parameters,
    rule: Rule,
}

impl Default for RuleInner {
    fn default() -> Self {
        RuleInner::Invocations(vec![])
    }
}

impl Rule {
    pub(crate) fn mesh(mesh: Mesh) -> Self {
        Self {
            inner: RuleInner::Mesh(mesh),
        }
    }

    pub fn push(self, transforms: &[Rc<Transform>], rule: Rule) -> Self {
        if let RuleInner::Invocations(mut invocations) = self.inner {
            Self {
                inner: RuleInner::Invocations({
                    invocations.push(InvocationBuilder {
                        transforms: transforms.iter().map(|t| t.clone()).collect(),
                        rule,
                    });
                    invocations
                }),
            }
        } else {
            self
        }
    }

    pub fn build(self, parameters: Parameters) -> Vec<Mesh> {
        Rule::compile(vec![], vec![], parameters, self)
    }

    fn expand(self, parameters: Parameters) -> Either<Mesh, Vec<Invocation>> {
        match self.inner {
            RuleInner::Invocations(invocation_builders) => Either::Right(
                invocation_builders
                    .into_iter()
                    .flat_map(|b| b.build(parameters))
                    .collect(),
            ),
            RuleInner::Mesh(mesh) => Either::Left(mesh.apply_parameters(parameters)),
        }
    }

    fn compile(
        mut meshes: Vec<Mesh>,
        mut invocations: Vec<Invocation>,
        parameters: Parameters,
        rule: Rule,
    ) -> Vec<Mesh> {
        match rule.expand(parameters) {
            Either::Left(mesh) => {
                meshes.push(mesh);
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
