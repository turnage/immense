use immense::*;
use std::fs::File;

fn main() {
    let meshes = rule![
        tf![
            Tf::saturation(0.8),
            Tf::hue(160.0),
            Replicate::n(36, vec![Tf::rz(10.0), Tf::ty(0.1)]),
            Replicate::n(36, vec![Tf::ry(10.0), Tf::tz(1.2), Tf::hue(3.4)]),
        ] => cube(),
    ]
    .generate();
    let mut output = File::create("torus.obj").expect("obj file");
    write_meshes(
        ExportConfig {
            grouping: MeshGrouping::ByColor,
            export_colors: Some(String::from("torus.mtl")),
        },
        meshes,
        &mut output,
    )
    .expect("rendered scene");
}
