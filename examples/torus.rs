use immense::*;
use rand::*;
use std::fs::File;
/*
rule grinder {
  36 * { rz 10 y 0.1 } 36 * { ry 10 z 1.2 b 0.99 h 12 } xbox
}*/

fn sanity_rings() -> Rule {
    Rule::new().push(
        cube()
            .tf(Replicate::n(36, Seq::new().push(Rotate::z(10.0)).push(Translate::y(0.1))))
            .tf(Replicate::n(36, Seq::new().push(Rotate::y(10.0)).push(Translate::z(1.2))))
            //.push(Scale::by(0.9))),
    )
}

fn main() {
    let mut output = File::create("torus.obj").expect("obj file");
    generate(sanity_rings(), &mut output).expect("rendered scene");
}
