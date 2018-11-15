use crate::api::{OutputMesh, Transform};
use failure_derive::Fail;
use std::fs::File;
use std::io;

#[derive(Fail, Debug)]
pub enum ExportError {
    #[fail(display = "Failed to write to obj file.")]
    ObjWriteError {
        #[cause]
        write_error: io::Error,
    },
    #[fail(display = "Failed to write to material file.")]
    MtlWriteError {
        #[cause]
        write_error: io::Error,
    },
}

/// A policy for grouping meshes in the object file.
///
/// Use this to specify how you want to work with your meshes later. E.g. if you want to use Blender
/// to procedurally material each mesh based on their location, you want
/// [MeshGrouping::Individual][MeshGrouping::Individual], but if you want to print the mesh with a
/// 3D printer, you want [MeshGrouping::AllTogether][MeshGrouping::AllTogether].
#[derive(Copy, Clone, Debug)]
pub enum MeshGrouping {
    /// All meshes will be combined into one object.
    AllTogether,
    /// Each mesh will be its own object.
    Individual,
    /// Each mesh is grouped with others of the same color.
    ByColor,
}

/// The default is [MeshGrouping::AllTogether][MeshGrouping::AllTogether].
impl Default for MeshGrouping {
    fn default() -> MeshGrouping {
        MeshGrouping::AllTogether
    }
}

/// Configuration for Wavefront object file output.
#[derive(Clone, Debug, Default)]
pub struct ExportConfig {
    /// Mesh grouping policy.
    pub grouping: MeshGrouping,
    /// Material definition sink to export colors to.
    ///
    /// This will write each color to a material lib file named by this parameter and reference
    /// those materials in the output object file.
    pub export_colors: Option<String>,
}

/// Writes out meshes as a Wavefront object file to the given [Write][io::Write] sink.
pub fn write_meshes(
    config: ExportConfig,
    meshes: impl Iterator<Item = OutputMesh>,
    mut sink: impl io::Write,
) -> Result<(), ExportError> {
    let mut mtl_file = if let Some(ref mtl_filename) = config.export_colors {
        let mtl_file = File::create(mtl_filename)
            .map_err(|write_error| ExportError::MtlWriteError { write_error })?;
        write!(&mut sink, "mtllib {}\n", mtl_filename)
            .map_err(|write_error| ExportError::ObjWriteError { write_error })?;
        Some(mtl_file)
    } else {
        None
    };
    let mut vertex_offset = 0;
    for mesh in meshes {
        let vertex_count = mesh.mesh.vertices().len();
        render_obj(&config, mesh, vertex_offset, &mut sink, mtl_file.as_mut())?;
        vertex_offset += vertex_count;
    }
    Ok(())
}

fn render_obj(
    config: &ExportConfig,
    output_mesh: OutputMesh,
    vertex_offset: usize,
    mut sink: impl io::Write,
    material_sink: Option<impl io::Write>,
) -> Result<(), ExportError> {
    let OutputMesh { transform, mesh } = output_mesh;
    let color = transform.unwrap_or(Transform::default()).get_color();
    let color_hex = format!("#{:x}", color.into_format::<u8>());
    match config.grouping {
        MeshGrouping::Individual => write!(&mut sink, "g g{}\n", vertex_offset)
            .map_err(|write_error| ExportError::ObjWriteError { write_error })?,
        MeshGrouping::ByColor => write!(&mut sink, "g {}\n", color_hex)
            .map_err(|write_error| ExportError::ObjWriteError { write_error })?,
        _ => (),
    };
    if let Some(mut material_sink) = material_sink {
        write!(&mut sink, "usemtl {}\n", color_hex)
            .map_err(|write_error| ExportError::ObjWriteError { write_error })?;
        write!(
            &mut material_sink,
            "newmtl {}\nKd {} {} {}\nillum 0\n",
            color_hex, color.red, color.green, color.blue
        )
        .map_err(|write_error| ExportError::MtlWriteError { write_error })?;
    }
    for vertex in mesh
        .vertices()
        .iter()
        .map(|v| transform.map(|t| t.apply_to(*v)).unwrap_or(*v))
    {
        write!(&mut sink, "v {} {} {}\n", vertex.x, vertex.y, vertex.z)
            .map_err(|write_error| ExportError::ObjWriteError { write_error })?;
    }
    for face in mesh.faces() {
        write!(&mut sink, "f ")
            .map_err(|write_error| ExportError::ObjWriteError { write_error })?;
        for vertex_index in *face {
            write!(&mut sink, " {}", vertex_index + vertex_offset)
                .map_err(|write_error| ExportError::ObjWriteError { write_error })?;
        }
        write!(&mut sink, "\n")
            .map_err(|write_error| ExportError::ObjWriteError { write_error })?;
    }
    Ok(())
}
