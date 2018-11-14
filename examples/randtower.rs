use immense::*;
use rand::*;
use std::fs::File;

#[derive(Debug)]
struct RandCube;

impl ToRule for RandCube {
    fn to_rule(&self) -> Rule {
        Rule::new().push(
            *thread_rng()
                .choose(&[Tf::tx(0.1), Tf::tx(-0.1), Tf::tx(0.2), Tf::tx(-0.2)])
                .unwrap(),
            cube(),
        )
    }
}

fn main() {
    let meshes = Rule::new()
        .push(Replicate::n(4, Tf::ty(1.0)), RandCube {})
        .generate();
    let mut output = File::create("randtower.obj").expect("obj file");
    write_meshes(ExportConfig::default(), meshes, &mut output).expect("rendered scene");
}
