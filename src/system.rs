use crate::mesh::Mesh;
use std::rc::Rc;

pub struct Parameters;

pub trait Rule {
    fn produce(&self, parameters: Parameters) -> Vec<Character>;
}

pub enum Character {
    Constant(Mesh),
    Variable {
        parameters: Parameters,
        rule: Rc<Rule>,
    },
}
