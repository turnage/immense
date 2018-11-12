use immense::*;
use std::fs::File;

fn grid2D(rows: usize, cols: usize) -> Rule {
    Rule::new()
        .push(cube())
        .tf(Replicate::n(rows, Translate::y(1.1)))
        .tf(Replicate::n(cols, Translate::x(1.1)))
}
fn main() {
    let mut output = File::create("grid2d.obj").expect("obj file");
    generate(grid2D(10, 5), &mut output).expect("rendered scene");
}
