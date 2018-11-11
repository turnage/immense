use immense::*;
use std::fs::File;
use std::rc::Rc;

struct Grid2D {
    rows: usize,
    cols: usize,
}

impl RuleBuilder for Grid2D {
    fn build_rule(&self, rule: Rule) -> Rule {
        rule.push(cube())
            .tf(Replicate::n(self.rows, Translate::y(1.1)))
            .tf(Replicate::n(self.cols, Translate::x(1.1)))
    }
}

fn main() {
    let mut output = File::create("grid2d.obj").expect("obj file");
    render_scene(Grid2D { rows: 10, cols: 5 }, &mut output).expect("rendered scene");
}
