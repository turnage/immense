use auto_from::auto_from;
use crate::mesh::Mesh;
use failure_derive::Fail;
use std::io;

#[auto_from]
#[derive(Fail, Debug)]
pub enum ExportError {
    #[fail(display = "Failed to write to obj file.")]
    WriteError {
        #[cause]
        write_error: io::Error,
    },
}

fn render_obj(
    mesh: &Mesh,
    vertex_offset: usize,
    mut sink: impl io::Write,
) -> Result<(), ExportError> {
    for vertex in &mesh.vertices {
        write!(&mut sink, "v {} {} {}\n", vertex.x, vertex.y, vertex.z)?;
    }
    for face in &mesh.faces {
        write!(&mut sink, "f ")?;
        for vertex_index in face {
            write!(&mut sink, " {}", vertex_index)?;
        }
        write!(&mut sink, "\n")?;
    }
    Ok(())
}
