//! Oxide-3D Geometry Importers & Exporters (STEP, STL, OBJ, 3MF, glTF).

use std::path::Path;
use thiserror::Error;

/// Geometry I/O error hierarchy.
#[derive(Debug, Error)]
pub enum IoFormatError {
    /// Format parse failure.
    #[error("Failed to parse format: {0}")]
    ParseError(String),

    /// Missing reader or adapter.
    #[error("Unsupported format feature: {0}")]
    Unsupported(String),

    /// System I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Triangulated mesh export/import data container.
#[derive(Debug, Default, Clone)]
pub struct TriangleMesh {
    /// Vertex positions [x, y, z].
    pub vertices: Vec<[f32; 3]>,
    /// Vertex normals [nx, ny, nz].
    pub normals: Vec<[f32; 3]>,
    /// Triangle index triplets.
    pub indices: Vec<[u32; 3]>,
}

/// Export a triangle mesh to binary STL format.
pub fn export_stl_binary<P: AsRef<Path>>(path: P, mesh: &TriangleMesh) -> Result<(), IoFormatError> {
    use std::io::Write;
    let mut file = std::fs::File::create(path)?;
    let header = [0u8; 80];
    file.write_all(&header)?;
    let num_triangles = mesh.indices.len() as u32;
    file.write_all(&num_triangles.to_le_bytes())?;

    for tri in &mesh.indices {
        let n = if !mesh.normals.is_empty() {
            mesh.normals[tri[0] as usize]
        } else {
            [0.0, 0.0, 1.0]
        };
        for f in n {
            file.write_all(&f.to_le_bytes())?;
        }
        for idx in *tri {
            let v = mesh.vertices[idx as usize];
            for f in v {
                file.write_all(&f.to_le_bytes())?;
            }
        }
        file.write_all(&[0u8, 0u8])?;
    }
    Ok(())
}
