use immense::*;
use std::fs::File;
use std::rc::Rc;

fn main() {
    let mut output = File::create("cube_stack.obj").expect("obj file");
    render_scene(
        Default::default(),
        Rule::default().push(&[Rc::new(Replicate::n(5, Translate::y(1.1f32)))], cube()),
        &mut output,
    )
    .expect("rendered scene");
}
