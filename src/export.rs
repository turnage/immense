// Copyright 2018 The immense Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::api::OutputMesh;
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

macro_rules! try_write_obj {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(ExportError::ObjWriteError { write_error: err }),
        }
    };
    ($expr:expr,) => {
        try!($expr)
    };
}

macro_rules! try_write_mtl {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(ExportError::MtlWriteError { write_error: err }),
        }
    };
    ($expr:expr,) => {
        try!($expr)
    };
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
        let mtl_file = try_write_mtl!(File::create(mtl_filename));
        try_write_obj!(write!(&mut sink, "mtllib {}\n", mtl_filename));
        Some(mtl_file)
    } else {
        None
    };
    let mut vertex_offset = 0;
    let mut normal_offset = 0;
    for mesh in meshes {
        let vertex_count = mesh.mesh().vertices().len();
        let normal_count = mesh.mesh().normals().map(|ns| ns.len()).unwrap_or(0);
        render_obj(
            &config,
            mesh,
            vertex_offset,
            normal_offset,
            &mut sink,
            mtl_file.as_mut(),
        )?;
        normal_offset += normal_count;
        vertex_offset += vertex_count;
    }
    Ok(())
}

fn render_obj(
    config: &ExportConfig,
    output_mesh: OutputMesh,
    vertex_offset: usize,
    normal_offset: usize,
    mut sink: impl io::Write,
    material_sink: Option<impl io::Write>,
) -> Result<(), ExportError> {
    let color = output_mesh.color();
    let color_hex = format!("#{:x}", color.into_format::<u8>());
    match config.grouping {
        MeshGrouping::Individual => try_write_obj!(write!(&mut sink, "g g{}\n", vertex_offset)),
        MeshGrouping::ByColor => try_write_obj!(write!(&mut sink, "g {}\n", color_hex)),
        _ => (),
    };
    if let Some(mut material_sink) = material_sink {
        try_write_obj!(write!(&mut sink, "usemtl {}\n", color_hex));
        try_write_mtl!(write!(
            &mut material_sink,
            "newmtl {}\nKd {} {} {}\nillum 0\n",
            color_hex, color.red, color.green, color.blue
        ));
    }
    for vertex in output_mesh.vertices() {
        try_write_obj!(write!(
            &mut sink,
            "v {} {} {}\n",
            vertex.x, vertex.y, vertex.z
        ));
    }

    if let Some(normals) = output_mesh.normals() {
        for normal in normals {
            try_write_obj!(write!(
                &mut sink,
                "vn {} {} {}\n",
                normal.x, normal.y, normal.z
            ));
        }
    }

    let write_face_vertex = |sink: &mut io::Write, vertex_index| -> Result<(), ExportError> {
        if let Some(_) = output_mesh.normals() {
            try_write_mtl!(write!(
                sink,
                " {}//{}",
                vertex_index + vertex_offset,
                vertex_index + normal_offset
            ));
        } else {
            try_write_mtl!(write!(sink, " {}", vertex_index + vertex_offset));
        };
        Ok(())
    };

    for face in output_mesh.faces() {
        try_write_obj!(write!(&mut sink, "f "));
        for vertex_index in face {
            write_face_vertex(&mut sink, vertex_index)?;
        }
        try_write_obj!(write!(&mut sink, "\n"));
    }
    Ok(())
}
