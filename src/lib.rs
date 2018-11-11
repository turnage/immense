#![feature(custom_attribute)]
#![feature(bind_by_move_pattern_guards)]

mod api;
mod error;
mod export;
mod mesh;
mod parameters;

pub use crate::api::*;
pub use crate::error::Error;
pub use crate::parameters::*;

use crate::error::Result;
use std::io;

pub fn render_scene(
    parameters: parameters::Parameters,
    rule: Rule,
    mut sink: impl io::Write,
) -> Result<()> {
    let meshes = rule.build(parameters);
    let mut vertex_offset = 0;
    for mesh in meshes {
        let vertex_count = mesh.vertices.len();
        export::render_obj(mesh, vertex_offset, &mut sink)?;
        vertex_offset += vertex_count;
    }
    Ok(())
}
