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

/// A policy for grouping meshes in the object file. Use this to specify how you want to work with
/// your meshes later. E.g. if you want to use Blender to procedurally material each mesh based on
/// their location, you want [MeshGrouping::Individual][MeshGrouping::Individual], but if you want
/// to print the mesh with a 3D printer, you want
/// [MeshGrouping::AllTogether][MeshGrouping::AllTogether].
#[derive(Copy, Clone, Debug)]
pub enum MeshGrouping {
    /// All meshes will be combined into one object.
    AllTogether,
    /// Each mesh will be its own object.
    Individual,
}

/// The default is [MeshGrouping::AllTogether][MeshGrouping::AllTogether].
impl Default for MeshGrouping {
    fn default() -> MeshGrouping {
        MeshGrouping::AllTogether
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ExportConfig {
    pub grouping: MeshGrouping,
}

pub fn render_obj(
    config: ExportConfig,
    mesh: Mesh,
    vertex_offset: usize,
    mut sink: impl io::Write,
) -> Result<(), ExportError> {
    match config.grouping {
        MeshGrouping::Individual => write!(&mut sink, "g g{}\n", vertex_offset)?,
        _ => (),
    };
    for vertex in &mesh.vertices {
        write!(&mut sink, "v {} {} {}\n", vertex.x, vertex.y, vertex.z)?;
    }
    for face in &mesh.faces {
        write!(&mut sink, "f ")?;
        for vertex_index in face {
            write!(&mut sink, " {}", vertex_index + vertex_offset)?;
        }
        write!(&mut sink, "\n")?;
    }
    Ok(())
}
