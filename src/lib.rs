#![feature(custom_attribute)]
#![feature(bind_by_move_pattern_guards)]

mod builtin;
mod error;
mod export;
mod mesh;
mod parameters;
mod system;

pub use crate::builtin::*;
pub use crate::error::Error;
pub use crate::parameters::*;
pub use crate::system::*;

use crate::error::Result;
use std::io;

pub fn render_scene(
    parameters: parameters::Parameters,
    producer: impl system::Producer,
    mut sink: impl io::Write,
) -> Result<()> {
    let meshes = system::compile(parameters, producer);
    let mut vertex_offset = 0;
    for mesh in meshes {
        let vertex_count = mesh.vertices.len();
        export::render_obj(mesh, vertex_offset, &mut sink)?;
        vertex_offset += vertex_count;
    }
    Ok(())
}
