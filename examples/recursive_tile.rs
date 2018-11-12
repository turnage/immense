use immense::*;
use std::fs::File;

fn recursive_tile(depth_budget: usize) -> Rule {
    let rule = Rule::new()
        .push(cube().tf(Translate::by(0.25, 0.25, 0.0)).tf(Scale::by(0.4)))
        .push(
            cube()
                .tf(Translate::by(-0.25, -0.25, 0.0))
                .tf(Scale::by(0.4)),
        )
        .push(
            cube()
                .tf(Translate::by(-0.25, 0.25, 0.0))
                .tf(Scale::by(0.4)),
        );
    if depth_budget > 0 {
        rule.push(
            recursive_tile(depth_budget - 1)
                .tf(Translate::by(0.25, -0.25, 0.0))
                .tf(Scale::by(0.5)),
        )
    } else {
        rule
    }
}

fn main() {
    let mut output = File::create("recursive_tile.obj").expect("obj file");
    generate(
        Rule::new()
            .push(cube().tf(Translate::x(1f32)))
            .push(recursive_tile(4)),
        &mut output,
    )
    .expect("rendered scene");
}
