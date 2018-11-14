use crate::api::{Rule, RuleInternal};
use crate::mesh::{vertex, Mesh};
use lazy_static::lazy_static;

pub fn cube() -> Rule {
    CUBE_RULE.clone()
}

lazy_static! {
    static ref CUBE_RULE: Rule = Rule::new().push_mesh(Mesh {
        vertices: vec![
            vertex(-0.5, 0.5, 0.5),
            vertex(-0.5, -0.5, 0.5),
            vertex(0.5, -0.5, 0.5),
            vertex(0.5, 0.5, 0.5),
            vertex(-0.5, 0.5, -0.5),
            vertex(-0.5, -0.5, -0.5),
            vertex(0.5, -0.5, -0.5),
            vertex(0.5, 0.5, -0.5),
        ],
        faces: vec![
            vec![1, 2, 3, 4],
            vec![8, 7, 6, 5],
            vec![4, 3, 7, 8],
            vec![5, 1, 4, 8],
            vec![5, 6, 2, 1],
            vec![2, 6, 7, 3],
        ],
    });
}
