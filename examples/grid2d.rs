use immense::*;
use std::fs::File;

struct Grid2D {
    rows: usize,
    cols: usize,
}

impl ToRule for Grid2D {
    fn to_rule(&self) -> Rule {
        rule![
            tf![
                Replicate::n(self.rows, Tf::ty(1.1)),
                Replicate::n(self.cols, Tf::tx(1.1)),
            ] =>
            cube(),
        ]
    }
}
fn main() {
    let mut output = File::create("grid2d.obj").expect("obj file");
    let meshes = Grid2D { rows: 2, cols: 2 }.to_rule().generate();
    write_meshes(ExportConfig::default(), meshes, &mut output).expect("rendered scene");
}
