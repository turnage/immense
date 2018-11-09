use immense::*;
use std::fs::File;
use std::rc::Rc;

struct Stack;

impl Producer for Stack {
    fn produce(&self, parameters: Parameters) -> Vec<Character> {
        vec![
            Character::Producer {
                parameters,
                producer: Rc::new(Cube {}),
            },
            Character::Producer {
                parameters: parameters.translate(0.0, 1.0, 0.0).scale(2.0),
                producer: Rc::new(Stack {}),
            },
        ]
    }
}

fn main() {
    let mut output = File::create("cube_stack.obj").expect("obj file");
    render_scene(Default::default(), Stack {}, &mut output).expect("rendered scene");
}
