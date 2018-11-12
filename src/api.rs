mod builtin;
mod transforms;

pub use self::builtin::*;
pub use self::transforms::*;

use crate::mesh::Mesh;
use crate::parameters::Parameters;
use itertools::Either;
use nalgebra::Matrix4;
use std::fmt;
use std::rc::Rc;

/// A mesh or a composition of subrules to expand until meshes are generated.
#[derive(Clone, Default)]
pub struct Rule {
    transforms: Seq,
    inner: RuleInner,
}

pub trait ToRule: fmt::Debug {
    fn to_rule(&self) -> Rule;
}

/// A trait for types that be converted into or produce ```Rule```s.
impl ToRule for Rule {
    fn to_rule(&self) -> Rule {
        self.clone()
    }
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Rule {{ transforms: {:?}, inner: {:?} }}",
            self.transforms.transform(identity()),
            self.inner
        )
    }
}

#[derive(Clone, Debug)]
enum RuleInner {
    Invocations(Vec<Rc<ToRule>>),
    Mesh(Mesh),
}

impl Default for RuleInner {
    fn default() -> Self {
        RuleInner::Invocations(vec![])
    }
}

#[derive(Debug)]
struct Invocation {
    parameters: Parameters,
    rule: Rule,
}

impl Rule {
    pub fn new() -> Self {
        Rule::default()
    }

    pub fn from(rule: impl ToRule + 'static) -> Self {
        Rule::new().push(rule)
    }

    pub fn mesh(mesh: Mesh) -> Self {
        Self {
            inner: RuleInner::Mesh(mesh),
            ..Default::default()
        }
    }

    pub fn tf(self, tf: impl Transform + 'static) -> Self {
        Self {
            transforms: self.transforms.push(tf),
            ..self
        }
    }

    pub fn push(self, rule: impl ToRule + 'static) -> Self {
        match self.inner {
            RuleInner::Invocations(mut invocations) => Self {
                inner: RuleInner::Invocations({
                    invocations.push(Rc::new(rule));
                    invocations
                }),
                ..self
            },
            inner @ RuleInner::Mesh(_) => Self {
                inner: RuleInner::Invocations(vec![Rc::new(Self { inner, ..self }), Rc::new(rule)]),
                ..Rule::default()
            },
        }
    }

    pub fn build(self, parameters: Parameters) -> Vec<Mesh> {
        Rule::compile(vec![], vec![], parameters, self)
    }

    fn expand(self, parameters: Parameters) -> Either<Vec<Mesh>, Vec<Invocation>> {
        let parameters: Vec<Parameters> = self
            .transforms
            .transform(parameters.transform)
            .iter()
            .map(|transform| Parameters {
                transform: parameters.transform * (*transform),
                ..parameters
            })
            .collect();
        match self.inner {
            RuleInner::Invocations(invocations) => Either::Right(
                invocations
                    .into_iter()
                    .flat_map(|rule| {
                        parameters
                            .iter()
                            .map(|parameters| Invocation {
                                parameters: *parameters,
                                rule: rule.to_rule(),
                            })
                            .collect::<Vec<Invocation>>()
                    })
                    .collect(),
            ),
            RuleInner::Mesh(mesh) => Either::Left(
                parameters
                    .iter()
                    .map(|parameters| mesh.clone().apply_parameters(*parameters))
                    .collect(),
            ),
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
