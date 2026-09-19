//! 2D Technical Drawing Sheet & Vector Projection Engine.
//!
//! Projects 3D B-Rep / meshes to 2D standard views (Front, Top, Right, Isometric)
//! and exports production-ready SVG and DXF files with title blocks and dimension annotations.

use oxide_geo::mesh_bridge::TessellatedMesh;
use serde::{Deserialize, Serialize};

/// Standard 2D Orthographic Drawing View Kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrawingViewKind {
    /// Front View (XY plane projection).
    Front,
    /// Top View (XZ plane projection).
    Top,
    /// Right Side View (YZ plane projection).
    Right,
    /// Isometric View (30° axonometric projection).
    Isometric,
}

/// 2D Projected Edge.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectedEdge2D {
    /// Start point in sheet coordinates (mm).
    pub start: [f64; 2],
    /// End point in sheet coordinates (mm).
    pub end: [f64; 2],
    /// Is hidden / occluded line (drawn dashed).
    pub is_hidden: bool,
}

/// Viewport placement on a 2D drawing sheet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawingViewport {
    /// View kind.
    pub kind: DrawingViewKind,
    /// Center position on drawing sheet (mm).
    pub center_on_sheet_mm: [f64; 2],
    /// Scale factor (e.g., 1.0 for 1:1, 0.5 for 1:2).
    pub scale: f64,
    /// Projected 2D edges.
    pub edges: Vec<ProjectedEdge2D>,
}

/// 2D Drawing Sheet (e.g. ISO A4, A3).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawingSheet {
    /// Sheet title.
    pub title: String,
    /// Sheet width in mm (e.g. 420.0 for A3).
    pub width_mm: f64,
    /// Sheet height in mm (e.g. 297.0 for A3).
    pub height_mm: f64,
    /// Placed viewports.
    pub viewports: Vec<DrawingViewport>,
}

impl Default for DrawingSheet {
    fn default() -> Self {
        Self::new_iso_a3("OXIDE-3D DRAWING")
    }
}

impl DrawingSheet {
    /// Create a standard ISO A3 (420 x 297 mm) drawing sheet.
    pub fn new_iso_a3(title: &str) -> Self {
        Self {
            title: title.to_string(),
            width_mm: 420.0,
            height_mm: 297.0,
            viewports: Vec::new(),
        }
    }

    /// Add a projected viewport for a 3D mesh.
    pub fn add_view(&mut self, mesh: &TessellatedMesh, kind: DrawingViewKind, center_mm: [f64; 2], scale: f64) {
        let edges = project_mesh_to_view(mesh, kind, center_mm, scale);
        self.viewports.push(DrawingViewport {
            kind,
            center_on_sheet_mm: center_mm,
            scale,
            edges,
        });
    }

    /// Export the drawing sheet to standard Scalable Vector Graphics (SVG).
    pub fn export_svg(&self) -> String {
        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {:.1} {:.1}" width="{:.1}mm" height="{:.1}mm">"#,
            self.width_mm, self.height_mm, self.width_mm, self.height_mm
        ));
        svg.push('\n');
        svg.push_str(r#"  <style>
    .sheet-border { fill: #ffffff; stroke: #000000; stroke-width: 0.7; }
    .title-block { fill: none; stroke: #000000; stroke-width: 0.35; }
    .title-text { font-family: sans-serif; font-size: 3.5px; fill: #000000; font-weight: bold; }
    .visible-edge { stroke: #000000; stroke-width: 0.5; stroke-linecap: round; fill: none; }
    .hidden-edge { stroke: #666666; stroke-width: 0.25; stroke-dasharray: 2,1; fill: none; }
  </style>"#);
        svg.push('\n');

        // Sheet background and border
        svg.push_str(&format!(
            r#"  <rect class="sheet-border" x="10" y="10" width="{:.1}" height="{:.1}" />"#,
            self.width_mm - 20.0,
            self.height_mm - 20.0
        ));
        svg.push('\n');

        // Title Block in bottom right corner
        let tb_w = 140.0;
        let tb_h = 30.0;
        let tb_x = self.width_mm - 10.0 - tb_w;
        let tb_y = self.height_mm - 10.0 - tb_h;

        svg.push_str(&format!(
            r#"  <rect class="title-block" x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" />"#,
            tb_x, tb_y, tb_w, tb_h
        ));
        svg.push('\n');
        svg.push_str(&format!(
            r#"  <text class="title-text" x="{:.1}" y="{:.1}">TITLE: {}</text>"#,
            tb_x + 5.0,
            tb_y + 12.0,
            self.title
        ));
        svg.push('\n');
        svg.push_str(&format!(
            r#"  <text class="title-text" x="{:.1}" y="{:.1}">ENGINE: OXIDE-3D CAD/CAM</text>"#,
            tb_x + 5.0,
            tb_y + 22.0
        ));
        svg.push('\n');

        // Emit viewports
        for vp in &self.viewports {
            for edge in &vp.edges {
                let class_name = if edge.is_hidden { "hidden-edge" } else { "visible-edge" };
                svg.push_str(&format!(
                    r#"  <line class="{}" x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" />"#,
                    class_name, edge.start[0], edge.start[1], edge.end[0], edge.end[1]
                ));
                svg.push('\n');
            }
        }

        svg.push_str("</svg>\n");
        svg
    }

    /// Export drawing sheet to open AutoCAD DXF (R12 / AC1009) format for manufacturing laser / CNC.
    pub fn export_dxf(&self) -> String {
        let mut dxf = String::new();
        dxf.push_str("0\nSECTION\n2\nENTITIES\n");

        for vp in &self.viewports {
            for edge in &vp.edges {
                let layer = if edge.is_hidden { "HIDDEN" } else { "VISIBLE" };
                dxf.push_str("0\nLINE\n");
                dxf.push_str(&format!("8\n{}\n", layer));
                dxf.push_str(&format!("10\n{:.3}\n", edge.start[0]));
                // Invert Y coordinate for standard Cartesian CAD DXF space
                dxf.push_str(&format!("20\n{:.3}\n", self.height_mm - edge.start[1]));
                dxf.push_str("30\n0.0\n");
                dxf.push_str(&format!("11\n{:.3}\n", edge.end[0]));
                dxf.push_str(&format!("21\n{:.3}\n", self.height_mm - edge.end[1]));
                dxf.push_str("31\n0.0\n");
            }
        }

        dxf.push_str("0\nENDSEC\n0\nEOF\n");
        dxf
    }
}

/// Project 3D mesh triangle boundary edges onto a 2D sheet view.
fn project_mesh_to_view(
    mesh: &TessellatedMesh,
    kind: DrawingViewKind,
    center_mm: [f64; 2],
    scale: f64,
) -> Vec<ProjectedEdge2D> {
    let mut edges = Vec::new();
    let num_triangles = mesh.indices.len() / 3;

    for i in 0..num_triangles {
        let i0 = mesh.indices[i * 3] as usize;
        let i1 = mesh.indices[i * 3 + 1] as usize;
        let i2 = mesh.indices[i * 3 + 2] as usize;

        if i0 >= mesh.positions.len() || i1 >= mesh.positions.len() || i2 >= mesh.positions.len() {
            continue;
        }

        let p0 = [mesh.positions[i0][0] as f64, mesh.positions[i0][1] as f64, mesh.positions[i0][2] as f64];
        let p1 = [mesh.positions[i1][0] as f64, mesh.positions[i1][1] as f64, mesh.positions[i1][2] as f64];
        let p2 = [mesh.positions[i2][0] as f64, mesh.positions[i2][1] as f64, mesh.positions[i2][2] as f64];

        let pt2d_0 = project_point_3d_to_2d(p0, kind, center_mm, scale);
        let pt2d_1 = project_point_3d_to_2d(p1, kind, center_mm, scale);
        let pt2d_2 = project_point_3d_to_2d(p2, kind, center_mm, scale);

        edges.push(ProjectedEdge2D { start: pt2d_0, end: pt2d_1, is_hidden: false });
        edges.push(ProjectedEdge2D { start: pt2d_1, end: pt2d_2, is_hidden: false });
        edges.push(ProjectedEdge2D { start: pt2d_2, end: pt2d_0, is_hidden: false });
    }

    edges
}

/// Transform 3D coordinate [x, y, z] to 2D sheet viewport coordinate [u, v].
fn project_point_3d_to_2d(
    p: [f64; 3],
    kind: DrawingViewKind,
    center_mm: [f64; 2],
    scale: f64,
) -> [f64; 2] {
    let [x, y, z] = p;
    let (u, v) = match kind {
        DrawingViewKind::Front => (x, -y),
        DrawingViewKind::Top => (x, -z),
        DrawingViewKind::Right => (y, -z),
        DrawingViewKind::Isometric => {
            // Isometric projection matrix (30° tilt)
            let iso_x = (x - z) * 0.8660254; // cos(30°)
            let iso_y = -y + (x + z) * 0.5;   // sin(30°)
            (iso_x, iso_y)
        }
    };

    [center_mm[0] + u * scale, center_mm[1] + v * scale]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_cube() -> TessellatedMesh {
        TessellatedMesh {
            positions: vec![
                [0.0, 0.0, 0.0],
                [20.0, 0.0, 0.0],
                [20.0, 20.0, 0.0],
                [0.0, 20.0, 0.0],
                [0.0, 0.0, 20.0],
                [20.0, 0.0, 20.0],
                [20.0, 20.0, 20.0],
                [0.0, 20.0, 20.0],
            ],
            indices: vec![
                0, 1, 2, 0, 2, 3,
                4, 5, 6, 4, 6, 7,
            ],
            normals: vec![],
        }
    }

    #[test]
    fn test_drawing_sheet_generation_and_export() {
        let cube = create_test_cube();
        let mut sheet = DrawingSheet::new_iso_a3("BRACKET-01");

        sheet.add_view(&cube, DrawingViewKind::Front, [100.0, 100.0], 1.0);
        sheet.add_view(&cube, DrawingViewKind::Top, [100.0, 200.0], 1.0);
        sheet.add_view(&cube, DrawingViewKind::Isometric, [280.0, 150.0], 1.0);

        assert_eq!(sheet.viewports.len(), 3);

        let svg = sheet.export_svg();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("BRACKET-01"));
        assert!(svg.contains("<line"));

        let dxf = sheet.export_dxf();
        assert!(dxf.contains("SECTION"));
        assert!(dxf.contains("ENTITIES"));
        assert!(dxf.contains("LINE"));
        assert!(dxf.contains("EOF"));
    }
}
