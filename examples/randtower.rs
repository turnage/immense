use immense::*;
use rand::*;
use std::fs::File;

#[derive(Debug)]
struct RandCube;

impl ToRule for RandCube {
    fn to_rule(&self) -> Rule {
        cube().tf(*thread_rng()
            .choose(&[
                Translate::x(0.1),
                Translate::x(-0.1),
                Translate::x(0.2),
                Translate::x(-0.2),
            ])
            .unwrap())
    }
}

fn main() {
    let mut output = File::create("randtower.obj").expect("obj file");
    generate(
        Rule::from(RandCube {}).tf(Replicate::n(4, Translate::y(1.0))),
        &mut output,
    )
    .expect("rendered scene");
}
