//! Oxide-3D Simulation Core Domain, Meshes, Boundary Conditions, and Materials.

use serde::{Deserialize, Serialize};

/// Type of finite element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElementKind {
    /// 4-node Linear Tetrahedron.
    Tet4,
    /// 10-node Quadratic Tetrahedron.
    Tet10,
    /// 8-node Linear Hexahedron (Brick).
    Hex8,
    /// 3-node Triangular Shell.
    Shell3,
}

/// Simulation volumetric/surface mesh.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SimulationMesh {
    /// Node coordinates [x, y, z].
    pub nodes: Vec<[f64; 3]>,
    /// Element node index lists.
    pub elements: Vec<Vec<usize>>,
    /// Element kind for each element.
    pub element_kinds: Vec<ElementKind>,
}

/// Isotropic linear elastic material properties.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LinearElasticMaterial {
    /// Young's modulus (in Pa, e.g. 69e9 for Al6061).
    pub youngs_modulus: f64,
    /// Poisson's ratio (e.g. 0.33).
    pub poissons_ratio: f64,
    /// Mass density in kg/m^3 (e.g. 2700.0).
    pub density: f64,
    /// Yield strength in Pa (e.g. 276e6).
    pub yield_strength: f64,
}

impl Default for LinearElasticMaterial {
    fn default() -> Self {
        // Structural Steel default
        Self {
            youngs_modulus: 200e9,
            poissons_ratio: 0.30,
            density: 7850.0,
            yield_strength: 250e6,
        }
    }
}
