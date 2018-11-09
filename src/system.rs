use crate::export::*;
use crate::mesh::Mesh;
use crate::parameters::Parameters;
use itertools::{Either, Itertools};
use std::rc::Rc;

pub trait Producer {
    fn produce(&self, parameters: Parameters) -> Vec<Character>;
}

pub enum Character {
    Constant(Mesh),
    Producer {
        parameters: Parameters,
        producer: Rc<Producer>,
    },
}

pub fn compile(parameters: Parameters, producer: impl Producer) -> Vec<Mesh> {
    let mut meshes = vec![];
    if parameters.depth_budget == 0 {
        return meshes;
    }

    let mut string = producer.produce(parameters);
    while !string.is_empty() {
        let mut next_level = vec![];
        for character in string {
            match character {
                Character::Constant(mesh) => meshes.push(mesh),
                Character::Producer {
                    parameters,
                    producer,
                } if parameters.depth_budget > 0 => {
                    next_level.append(&mut producer.produce(Parameters {
                        depth_budget: parameters.depth_budget - 1,
                        ..parameters
                    }))
                }
                _ => (),
            }
        }
        string = next_level;
    }
    meshes
}
