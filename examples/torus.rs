use immense::*;
use std::fs::File;

fn main() {
    let meshes = Rule::new()
        .push(
            vec![
                Replicate::n(36, vec![Tf::rz(10.0), Tf::ty(0.1)]),
                Replicate::n(36, vec![Tf::ry(10.0), Tf::tz(1.2)]),
            ],
            cube(),
        )
        .generate();
    let mut output = File::create("torus.obj").expect("obj file");
    write_meshes(ExportConfig::default(), meshes, &mut output).expect("rendered scene");
}
