//! Oxide-3D Test Utilities, Fixtures, Golden Files, and Geometry Validators.

use oxide_geo::topology::TopologyDatabase;

/// Validates Euler-Poincaré formula for a simple closed polyhedron (V - E + F = 2).
#[must_use]
pub fn check_euler_poincare(db: &TopologyDatabase) -> bool {
    let v = db.vertices.len() as i64;
    let e = db.edges.len() as i64;
    let f = db.faces.len() as i64;
    if v == 0 && e == 0 && f == 0 {
        return true;
    }
    (v - e + f) == 2
}
