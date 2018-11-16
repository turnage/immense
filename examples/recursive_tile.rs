use immense::*;
use std::fs::File;

struct RecursiveTile {
    depth_budget: usize,
}

impl ToRule for RecursiveTile {
    fn to_rule(&self) -> Rule {
        let rule = rule![
            tf![Tf::t(0.25, 0.25, 0.0), Tf::s(0.4)] => icosphere(),
            tf![Tf::t(-0.25, -0.25, 0.0), Tf::s(0.4)] => icosphere(),
            tf![Tf::t(-0.25, 0.25, 0.0), Tf::s(0.4)] => icosphere(),
        ];
        if self.depth_budget > 0 {
            rule.push(
                vec![Tf::t(0.25, -0.25, 0.0), Tf::s(0.4)],
                RecursiveTile {
                    depth_budget: self.depth_budget - 1,
                },
            )
        } else {
            rule
        }
    }
}

fn main() {
    let meshes = Rule::new()
        .push(None, icosphere())
        .push(Tf::tx(1.0), cube())
        .generate(); //RecursiveTile { depth_budget: 3 }.to_rule().generate();
    let mut output = File::create("recursive_tile.obj").expect("obj file");
    write_meshes(ExportConfig::default(), meshes, &mut output).expect("rendered scene");
}
