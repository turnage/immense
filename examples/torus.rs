use immense::*;
use std::fs::File;

fn main() {
    let mut output = File::create("torus.obj").expect("obj file");
    let mut sys = System::new();
    let cube = sys.define(Rule::new().invoke(
        vec![
            Replicate::n(36, vec![Tf::rz(10.0), Tf::ty(0.1)]),
            Replicate::n(36, vec![Tf::ry(10.0), Tf::tz(1.2)]),
        ],
        cube(),
    ));
    let meshes = sys.generate(cube);
    write_meshes(meshes, &mut output).expect("rendered scene");
}
