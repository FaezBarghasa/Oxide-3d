//! Oxide-3D Geometry Importers & Exporters (STEP, STL, OBJ, 3MF, glTF).

use std::io::{BufRead, BufReader, Write};
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
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TriangleMesh {
    /// Vertex positions [x, y, z].
    pub vertices: Vec<[f32; 3]>,
    /// Vertex normals [nx, ny, nz].
    pub normals: Vec<[f32; 3]>,
    /// Triangle index triplets.
    pub indices: Vec<[u32; 3]>,
}

/// Export a triangle mesh to binary STL format.
pub fn export_stl_binary<P: AsRef<Path>>(
    path: P,
    mesh: &TriangleMesh,
) -> Result<(), IoFormatError> {
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

/// Export a triangle mesh to Wavefront OBJ format.
pub fn export_obj<P: AsRef<Path>>(path: P, mesh: &TriangleMesh) -> Result<(), IoFormatError> {
    let mut file = std::fs::File::create(path)?;

    writeln!(file, "# Oxide-3D Wavefront OBJ Exporter")?;

    for v in &mesh.vertices {
        writeln!(file, "v {:.6} {:.6} {:.6}", v[0], v[1], v[2])?;
    }

    for n in &mesh.normals {
        writeln!(file, "vn {:.6} {:.6} {:.6}", n[0], n[1], n[2])?;
    }

    let has_normals = !mesh.normals.is_empty();

    for tri in &mesh.indices {
        // OBJ indices are 1-based
        let i0 = tri[0] + 1;
        let i1 = tri[1] + 1;
        let i2 = tri[2] + 1;
        if has_normals {
            writeln!(file, "f {}//{} {}//{} {}//{}", i0, i0, i1, i1, i2, i2)?;
        } else {
            writeln!(file, "f {} {} {}", i0, i1, i2)?;
        }
    }

    Ok(())
}

/// Import a triangle mesh from Wavefront OBJ format.
pub fn import_obj<P: AsRef<Path>>(path: P) -> Result<TriangleMesh, IoFormatError> {
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);

    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "v" => {
                if parts.len() >= 4 {
                    let x: f32 = parts[1]
                        .parse()
                        .map_err(|e| IoFormatError::ParseError(format!("{e}")))?;
                    let y: f32 = parts[2]
                        .parse()
                        .map_err(|e| IoFormatError::ParseError(format!("{e}")))?;
                    let z: f32 = parts[3]
                        .parse()
                        .map_err(|e| IoFormatError::ParseError(format!("{e}")))?;
                    vertices.push([x, y, z]);
                }
            }
            "vn" => {
                if parts.len() >= 4 {
                    let nx: f32 = parts[1]
                        .parse()
                        .map_err(|e| IoFormatError::ParseError(format!("{e}")))?;
                    let ny: f32 = parts[2]
                        .parse()
                        .map_err(|e| IoFormatError::ParseError(format!("{e}")))?;
                    let nz: f32 = parts[3]
                        .parse()
                        .map_err(|e| IoFormatError::ParseError(format!("{e}")))?;
                    normals.push([nx, ny, nz]);
                }
            }
            "f" => {
                if parts.len() >= 4 {
                    let parse_idx = |token: &str| -> Result<u32, IoFormatError> {
                        let v_part = token.split('/').next().unwrap_or("1");
                        let idx: u32 = v_part
                            .parse()
                            .map_err(|e| IoFormatError::ParseError(format!("{e}")))?;
                        Ok(idx.saturating_sub(1)) // convert 1-based to 0-based
                    };

                    let idx0 = parse_idx(parts[1])?;
                    let idx1 = parse_idx(parts[2])?;
                    let idx2 = parse_idx(parts[3])?;
                    indices.push([idx0, idx1, idx2]);

                    // If quad or higher polygon, triangulate as fan
                    for i in 4..parts.len() {
                        let prev_idx = parse_idx(parts[i - 1])?;
                        let curr_idx = parse_idx(parts[i])?;
                        indices.push([idx0, prev_idx, curr_idx]);
                    }
                }
            }
            _ => {}
        }
    }

    Ok(TriangleMesh {
        vertices,
        normals,
        indices,
    })
}

/// Simplified STEP (ISO 10303-21) neutral exchange entity parser.
#[derive(Debug, Clone, PartialEq)]
pub struct StepEntity {
    /// Step entity id (e.g. #120).
    pub id: u64,
    /// Entity type name (e.g. "CARTESIAN_POINT", "MANIFOLD_SOLID_BREP").
    pub name: String,
    /// Parameter string literal inside parentheses.
    pub params: String,
}

/// Parse ISO 10303-21 STEP physical file lines into entity records.
pub fn parse_step_entities(content: &str) -> Vec<StepEntity> {
    let mut entities = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            if let Some(eq_pos) = trimmed.find('=') {
                let id_str = &trimmed[1..eq_pos].trim();
                if let Ok(id) = id_str.parse::<u64>() {
                    let rest = trimmed[eq_pos + 1..].trim().trim_end_matches(';');
                    if let Some(paren_pos) = rest.find('(') {
                        let name = rest[..paren_pos].trim().to_string();
                        let params = rest[paren_pos + 1..].trim_end_matches(')').to_string();
                        entities.push(StepEntity { id, name, params });
                    }
                }
            }
        }
    }

    entities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obj_export_and_import() {
        let mesh = TriangleMesh {
            vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            normals: vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
            indices: vec![[0, 1, 2]],
        };

        let temp_dir = std::env::temp_dir();
        let obj_path = temp_dir.join("oxide_test_mesh.obj");

        export_obj(&obj_path, &mesh).expect("OBJ export should succeed");
        let imported = import_obj(&obj_path).expect("OBJ import should succeed");

        assert_eq!(imported.vertices.len(), 3);
        assert_eq!(imported.indices.len(), 1);
        assert_eq!(imported.indices[0], [0, 1, 2]);

        let _ = std::fs::remove_file(obj_path);
    }

    #[test]
    fn test_step_entity_parsing() {
        let step_data = r#"
ISO-10303-21;
HEADER;
ENDSEC;
DATA;
#10 = CARTESIAN_POINT('Origin', (0.0, 0.0, 0.0));
#20 = DIRECTION('Z_Axis', (0.0, 0.0, 1.0));
#30 = AXIS2_PLACEMENT_3D('Ref', #10, #20, $);
ENDSEC;
END-ISO-10303-21;
        "#;

        let entities = parse_step_entities(step_data);
        assert_eq!(entities.len(), 3);
        assert_eq!(entities[0].id, 10);
        assert_eq!(entities[0].name, "CARTESIAN_POINT");
        assert_eq!(entities[1].id, 20);
        assert_eq!(entities[1].name, "DIRECTION");
        assert_eq!(entities[2].id, 30);
        assert_eq!(entities[2].name, "AXIS2_PLACEMENT_3D");
    }
}
